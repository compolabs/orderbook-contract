import {
  arrayify,
  CoinQuantityLike,
  hashMessage,
  WalletLocked,
  WalletUnlocked,
} from "fuels";

import { DEFAULT_DECIMALS, FAUCET_AMOUNTS } from "../../constants";
import { Token } from "../../entity";
import BN from "../../utils/BN";

import { AccountBalanceAbi__factory } from "./types/account-balance";
import { ClearingHouseAbi__factory } from "./types/clearing-house";
import { OrderbookAbi__factory } from "./types/orderbook";
import { AssetIdInput, I64Input } from "./types/orderbook/OrderbookAbi";
import { PerpMarketAbi__factory } from "./types/perp-market";
import { ProxyAbi__factory } from "./types/proxy";
import { PythContractAbi__factory } from "./types/pyth";
import { TokenAbi__factory } from "./types/src-20";
import { IdentityInput } from "./types/src-20/TokenAbi";
import { VaultAbi__factory } from "./types/vault";
import {
  CONTRACT_ADDRESSES,
  GAS_LIMIT,
  TOKENS_BY_ASSET_ID,
  TOKENS_BY_SYMBOL,
} from "./constants";
import { Fetch } from "./Fetch";

export class Api {
  public fetch = new Fetch();

  createSpotOrder = async (
    baseToken: Token,
    quoteToken: Token,
    size: string,
    price: string,
    wallet: WalletLocked | WalletUnlocked,
  ): Promise<string> => {
    const orderbookFactory = OrderbookAbi__factory.connect(
      CONTRACT_ADDRESSES.spotMarket,
      wallet,
    );

    const assetId: AssetIdInput = { value: baseToken.assetId };
    const isNegative = size.includes("-");
    const absSize = size.replace("-", "");
    const baseSize: I64Input = { value: absSize, negative: isNegative };

    const amountToSend = new BN(absSize)
      .times(price)
      .dividedToIntegerBy(
        new BN(10).pow(
          DEFAULT_DECIMALS + baseToken.decimals - quoteToken.decimals,
        ),
      );

    const forward: CoinQuantityLike = {
      amount: isNegative ? absSize : amountToSend.toString(),
      assetId: isNegative ? baseToken.assetId : quoteToken.assetId,
    };

    const tx = await orderbookFactory.functions
      .open_order(assetId, baseSize, price)
      .callParams({ forward })
      .txParams({ gasPrice: 1, gasLimit: GAS_LIMIT })
      .call();

    return tx.transactionId;
  };

  cancelSpotOrder = async (
    orderId: string,
    wallet: WalletLocked | WalletUnlocked,
  ): Promise<void> => {
    const orderbookFactory = OrderbookAbi__factory.connect(
      CONTRACT_ADDRESSES.spotMarket,
      wallet,
    );

    await orderbookFactory.functions
      .cancel_order(orderId)
      .txParams({ gasPrice: 1, gasLimit: GAS_LIMIT })
      .call();
  };

  mintToken = async (
    assetAddress: string,
    wallet: WalletLocked | WalletUnlocked,
    mintAmount?: number
  ): Promise<void> => {
    const tokenFactory = CONTRACT_ADDRESSES.tokenFactory;
    const tokenFactoryContract = TokenAbi__factory.connect(
      tokenFactory,
      wallet,
    );

    const token = TOKENS_BY_ASSET_ID[assetAddress];
    const amount = mintAmount ?? BN.parseUnits(
      FAUCET_AMOUNTS[token.symbol].toString(),
      token.decimals,
    );
    const hash = hashMessage(token.symbol);
    const identity: IdentityInput = {
      Address: {
        value: wallet.address.toB256(),
      },
    };

    await tokenFactoryContract.functions
      .mint(identity, hash, amount.toString())
      .txParams({ gasPrice: 1, gasLimit: GAS_LIMIT })
      .call();
  };

  approve = async (assetAddress: string, amount: string): Promise<void> => {};

  allowance = async (assetAddress: string): Promise<string> => {
    return "";
  };

  depositPerpCollateral = async (
    assetAddress: string,
    amount: string,
    wallet: WalletLocked | WalletUnlocked,
  ) => {
    const vaultFactory = VaultAbi__factory.connect(
      CONTRACT_ADDRESSES.vault,
      wallet,
    );

    const assetIdInput: AssetIdInput = {
      value: assetAddress,
    };

    const forward: CoinQuantityLike = {
      assetId: assetAddress,
      amount,
    };

    await vaultFactory.functions
      .deposit_collateral(assetIdInput)
      .callParams({ forward })
      .txParams({ gasPrice: 1, gasLimit: GAS_LIMIT })
      .call();
  };

  withdrawPerpCollateral = async (
    assetAddress: string,
    amount: string,
    updateData: string[],
    wallet: WalletLocked | WalletUnlocked,
  ) => {
    const vaultFactory = VaultAbi__factory.connect(
      CONTRACT_ADDRESSES.vault,
      wallet,
    );

    const eth = TOKENS_BY_SYMBOL["ETH"];

    const assetIdInput: AssetIdInput = {
      value: assetAddress,
    };

    const parsedUpdateData = updateData.map((v) => Array.from(arrayify(v)));

    const forward: CoinQuantityLike = {
      amount: "10",
      assetId: eth.assetId,
    };

    await vaultFactory.functions
      .withdraw_collateral(amount, assetIdInput, parsedUpdateData)
      .callParams({ forward })
      .txParams({ gasPrice: 1, gasLimit: GAS_LIMIT })
      .addContracts([
        ProxyAbi__factory.connect(CONTRACT_ADDRESSES.proxy, wallet),
        PerpMarketAbi__factory.connect(CONTRACT_ADDRESSES.perpMarket, wallet),
        AccountBalanceAbi__factory.connect(
          CONTRACT_ADDRESSES.accountBalance,
          wallet,
        ),
        ClearingHouseAbi__factory.connect(
          CONTRACT_ADDRESSES.clearingHouse,
          wallet,
        ),
        VaultAbi__factory.connect(CONTRACT_ADDRESSES.vault, wallet),
        PythContractAbi__factory.connect(CONTRACT_ADDRESSES.pyth, wallet),
      ])
      .call();
  };

  openPerpOrder = async (
    assetAddress: string,
    amount: string,
    price: string,
    updateData: string[],
    wallet: WalletLocked | WalletUnlocked,
  ): Promise<void> => {
    const clearingHouseFactory = ClearingHouseAbi__factory.connect(
      CONTRACT_ADDRESSES.clearingHouse,
      wallet,
    );

    const eth = TOKENS_BY_SYMBOL["ETH"];

    const assetIdInput: AssetIdInput = {
      value: assetAddress,
    };

    const vaultFactory = VaultAbi__factory.connect(
      CONTRACT_ADDRESSES.vault,
      wallet,
    );

    const free = await vaultFactory.functions
      .get_free_collateral({
        value: wallet.address.toB256(),
      })
      .get();

    const market = await clearingHouseFactory.functions
      .get_market(assetIdInput)
      .get();

    const isNegative = amount.includes("-");
    const absSize = amount.replace("-", "");
    const baseSize: I64Input = { value: absSize, negative: isNegative };

    const parsedUpdateData = updateData.map((v) => Array.from(arrayify(v)));

    const forward: CoinQuantityLike = {
      amount: "10",
      assetId: eth.assetId,
    };

    await clearingHouseFactory.functions
      .open_order(assetIdInput, baseSize, price, parsedUpdateData)
      .callParams({ forward })
      .txParams({ gasPrice: 1, gasLimit: GAS_LIMIT })
      .addContracts([
        ProxyAbi__factory.connect(CONTRACT_ADDRESSES.proxy, wallet),
        PerpMarketAbi__factory.connect(CONTRACT_ADDRESSES.perpMarket, wallet),
        AccountBalanceAbi__factory.connect(
          CONTRACT_ADDRESSES.accountBalance,
          wallet,
        ),
        VaultAbi__factory.connect(CONTRACT_ADDRESSES.vault, wallet),
        PythContractAbi__factory.connect(CONTRACT_ADDRESSES.pyth, wallet),
      ])
      .call();
  };

  removePerpOrder = async (
    orderId: string,
    wallet: WalletLocked | WalletUnlocked,
  ): Promise<void> => {
    const clearingHouseFactory = ClearingHouseAbi__factory.connect(
      CONTRACT_ADDRESSES.clearingHouse,
      wallet,
    );

    await clearingHouseFactory.functions
      .remove_order(orderId)
      .txParams({ gasPrice: 1, gasLimit: GAS_LIMIT })
      .addContracts([
        ProxyAbi__factory.connect(CONTRACT_ADDRESSES.proxy, wallet),
        PerpMarketAbi__factory.connect(CONTRACT_ADDRESSES.perpMarket, wallet),
        ClearingHouseAbi__factory.connect(
          CONTRACT_ADDRESSES.clearingHouse,
          wallet,
        ),
      ])
      .call();
  };
}
