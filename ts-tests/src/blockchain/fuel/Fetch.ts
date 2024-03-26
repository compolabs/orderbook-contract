import { Address, Bech32Address, WalletLocked, WalletUnlocked } from "fuels";

import { SpotMarketOrder, SpotMarketTrade, Token } from "../../entity";
import { PerpMarket } from "../../entity/PerpMarket";
import { PerpOrder } from "../../entity/PerpOrder";
import { PerpPosition } from "../../entity/PerpPosition";
import BN from "../../utils/BN";
import {
  FetchOrdersParams,
  FetchTradesParams,
  MarketCreateEvent,
  SpotMarketVolume,
} from "../types";

import { AccountBalanceAbi__factory } from "./types/account-balance";
import { ClearingHouseAbi__factory } from "./types/clearing-house";
import { OrderbookAbi__factory } from "./types/orderbook";
import { PerpMarketAbi__factory } from "./types/perp-market";
import { VaultAbi__factory } from "./types/vault";
import { AddressInput, AssetIdInput } from "./types/vault/VaultAbi";
import { CONTRACT_ADDRESSES, TOKENS_BY_SYMBOL } from "./constants";
import { convertI64ToBn } from "./utils";

export class Fetch {
  fetchSpotMarkets = async (
    limit: number,
    tokens: Token[],
    wallet: WalletLocked | WalletUnlocked,
  ): Promise<MarketCreateEvent[]> => {
    const orderbookFactory = OrderbookAbi__factory.connect(
      CONTRACT_ADDRESSES.spotMarket,
      wallet,
    );

    const getMarketByIdPromises = tokens.map((t) =>
      orderbookFactory.functions
        .get_market_by_id({
          value: t.assetId,
        })
        .get(),
    );

    const data = await Promise.all(getMarketByIdPromises);

    const markets = data.map((market) => ({
      id: market.value.asset_id.value,
      assetId: market.value.asset_id.value,
      decimal: market.value.asset_decimals,
    }));

    return markets;
  };

  fetchSpotMarketPrice = async (baseToken: string): Promise<BN> => {
    console.warn("[fetchMarketPrice] NOT IMPLEMENTED FOR FUEL");
    return BN.ZERO;
  };

  fetchSpotOrders = async (
    { baseToken, type, limit, trader, isActive }: FetchOrdersParams,
    wallet: WalletLocked | WalletUnlocked,
  ): Promise<SpotMarketOrder[]> => {
    const orderbookFactory = OrderbookAbi__factory.connect(
      CONTRACT_ADDRESSES.spotMarket,
      wallet,
    );

    // TODO: Should be fixed. Can't get all trades, only for trader.
    if (!trader) return [];

    const ordersId = await orderbookFactory.functions
      .orders_by_trader({
        value: new Address(trader as Bech32Address).toB256(),
      })
      .get();

    const ordersPromises = ordersId.value.map((order) =>
      orderbookFactory.functions.order_by_id(order).get(),
    );

    const data = await Promise.all(ordersPromises);

    const dataFiltered = data.filter(({ value }) => {
      if (value?.base_token.value !== baseToken) return false;
      if (value?.base_size.negative && type && type !== "SELL") return false;

      return true;
    });

    const orders = dataFiltered.map(({ value }) => {
      const baseSizeBn = new BN(value!.base_size.value.toString());
      const baseSize = value!.base_size.negative
        ? baseSizeBn.times(-1)
        : baseSizeBn;
      return new SpotMarketOrder({
        id: value!.id,
        baseToken: value!.base_token.value,
        trader: value!.trader.value,
        baseSize: baseSize.toNumber(),
        orderPrice: value!.base_price.toNumber(),
        // TODO: Fetch date somehow
        blockTimestamp: Date.now(),
      });
    });

    return orders;
  };

  fetchSpotTrades = async ({
    baseToken,
    limit,
    trader,
  }: FetchTradesParams): Promise<SpotMarketTrade[]> => {
    console.warn("[fetchTrades] NOT IMPLEMENTED FOR FUEL");
    return [];
  };

  fetchSpotVolume = async (): Promise<SpotMarketVolume> => {
    console.warn("[fetchVolume] NOT IMPLEMENTED FOR FUEL");
    return { volume: BN.ZERO, high: BN.ZERO, low: BN.ZERO };
  };

  fetchPerpCollateralBalance = async (
    accountAddress: string,
    assetAddress: string,
    wallet: WalletLocked | WalletUnlocked,
  ): Promise<BN> => {
    const vaultFactory = VaultAbi__factory.connect(
      CONTRACT_ADDRESSES.vault,
      wallet,
    );

    const addressInput: AddressInput = {
      value: new Address(accountAddress as any).toB256(),
    };

    const assetIdInput: AssetIdInput = {
      value: assetAddress,
    };

    const result = await vaultFactory.functions
      .get_collateral_balance(addressInput, assetIdInput)
      .get();

    const collateralBalance = new BN(result.value.toString());

    return collateralBalance;
  };

  fetchPerpAllTraderPositions = async (
    accountAddress: string,
    wallet: WalletLocked | WalletUnlocked,
  ): Promise<PerpPosition[]> => {
    const accountBalanceFactory = AccountBalanceAbi__factory.connect(
      CONTRACT_ADDRESSES.accountBalance,
      wallet,
    );

    const addressInput: AddressInput = {
      value: new Address(accountAddress as any).toB256(),
    };

    const result = await accountBalanceFactory.functions
      .get_all_trader_positions(addressInput)
      .get();

    const positions = result.value.map(
      ([assetAddress, accountBalance]) =>
        new PerpPosition({
          baseTokenAddress: assetAddress.value,
          lastTwPremiumGrowthGlobal: convertI64ToBn(
            accountBalance.last_tw_premium_growth_global,
          ),
          takerOpenNational: convertI64ToBn(accountBalance.taker_open_notional),
          takerPositionSize: convertI64ToBn(accountBalance.taker_position_size),
        }),
    );

    return positions;
  };

  fetchPerpMarketPrice = async (
    assetAddress: string,
    wallet: WalletLocked | WalletUnlocked,
  ): Promise<BN> => {
    const perpMarketFactory = PerpMarketAbi__factory.connect(
      CONTRACT_ADDRESSES.perpMarket,
      wallet,
    );

    const assetIdInput: AssetIdInput = {
      value: assetAddress,
    };

    const result = await perpMarketFactory.functions
      .get_market_price(assetIdInput)
      .get();

    const marketPrice = new BN(result.value.toString());

    return marketPrice;
  };

  fetchPerpFundingRate = async (
    assetAddress: string,
    wallet: WalletLocked | WalletUnlocked,
  ): Promise<BN> => {
    const accountBalanceFactory = AccountBalanceAbi__factory.connect(
      CONTRACT_ADDRESSES.accountBalance,
      wallet,
    );

    const assetIdInput: AssetIdInput = {
      value: assetAddress,
    };

    const result = await accountBalanceFactory.functions
      .get_funding_rate(assetIdInput)
      .get();

    const fundingRate = convertI64ToBn(result.value);

    return fundingRate;
  };

  fetchPerpFreeCollateral = async (
    accountAddress: string,
    wallet: WalletLocked | WalletUnlocked,
  ): Promise<any> => {
    const vaultFactory = VaultAbi__factory.connect(
      CONTRACT_ADDRESSES.vault,
      wallet,
    );

    const addressInput: AddressInput = {
      value: new Address(accountAddress as any).toB256(),
    };

    const result = await vaultFactory.functions
      .get_free_collateral(addressInput)
      .get();

    const freeCollateral = new BN(result.value.toString());

    return freeCollateral;
  };

  fetchPerpMarket = async (
    assetAddress: string,
    wallet: WalletLocked | WalletUnlocked,
  ): Promise<PerpMarket> => {
    const clearingHouseFactory = ClearingHouseAbi__factory.connect(
      CONTRACT_ADDRESSES.clearingHouse,
      wallet,
    );

    const assetIdInput: AssetIdInput = {
      value: assetAddress,
    };

    const result = await clearingHouseFactory.functions
      .get_market(assetIdInput)
      .get();

    const pausedIndexPrice = result.value.paused_index_price
      ? new BN(result.value.paused_index_price.toString())
      : undefined;
    const pausedTimestamp = result.value.paused_timestamp
      ? result.value.paused_timestamp.toNumber()
      : undefined;
    const closedPrice = result.value.closed_price
      ? new BN(result.value.closed_price.toString())
      : undefined;

    const perpMarket = new PerpMarket({
      baseTokenAddress: result.value.asset_id.value,
      quoteTokenAddress: TOKENS_BY_SYMBOL["USDC"].assetId,
      imRatio: new BN(result.value.im_ratio.toString()),
      mmRatio: new BN(result.value.mm_ratio.toString()),
      status: result.value.status,
      pausedIndexPrice,
      pausedTimestamp,
      closedPrice,
    });

    return perpMarket;
  };

  fetchPerpPendingFundingPayment = async (
    accountAddress: string,
    assetAddress: string,
    wallet: WalletLocked | WalletUnlocked,
  ): Promise<{ fundingPayment: BN; fundingGrowthPayment: BN }> => {
    const accountBalanceFactory = AccountBalanceAbi__factory.connect(
      CONTRACT_ADDRESSES.accountBalance,
      wallet,
    );

    const addressInput: AddressInput = {
      value: new Address(accountAddress as any).toB256(),
    };

    const assetIdInput: AssetIdInput = {
      value: assetAddress,
    };

    const result = await accountBalanceFactory.functions
      .get_pending_funding_payment(addressInput, assetIdInput)
      .get();

    const fundingPayment = convertI64ToBn(result.value[0]);
    const fundingGrowthPayment = convertI64ToBn(result.value[1]);

    return { fundingPayment, fundingGrowthPayment };
  };

  fetchPerpIsAllowedCollateral = async (
    assetAddress: string,
    wallet: WalletLocked | WalletUnlocked,
  ): Promise<boolean> => {
    const vaultFactory = VaultAbi__factory.connect(
      CONTRACT_ADDRESSES.vault,
      wallet,
    );

    const assetIdInput: AssetIdInput = {
      value: assetAddress,
    };

    const result = await vaultFactory.functions
      .is_allowed_collateral(assetIdInput)
      .get();

    return result.value;
  };

  fetchPerpTraderOrders = async (
    accountAddress: string,
    assetAddress: string,
    wallet: WalletLocked | WalletUnlocked,
  ): Promise<PerpOrder[]> => {
    const vaultFactory = PerpMarketAbi__factory.connect(
      CONTRACT_ADDRESSES.perpMarket,
      wallet,
    );

    const addressInput: AddressInput = {
      value: new Address(accountAddress as any).toB256(),
    };

    const assetIdInput: AssetIdInput = {
      value: assetAddress,
    };

    const result = await vaultFactory.functions
      .get_trader_orders(addressInput, assetIdInput)
      .get();

    const orders = result.value.map(
      (order) =>
        new PerpOrder({
          id: order.id,
          baseSize: convertI64ToBn(order.base_size),
          baseTokenAddress: order.base_token.value,
          orderPrice: new BN(order.order_price.toString()),
          trader: order.trader.value,
        }),
    );

    return orders;
  };
}
