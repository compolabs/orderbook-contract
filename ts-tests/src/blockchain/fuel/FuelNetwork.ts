import { Provider, Wallet } from "fuels";
import { makeObservable } from "mobx";
import { Nullable } from "tsdef";

import { SpotMarketOrder, SpotMarketTrade } from "../../entity";
import { Token } from "../../entity/Token";
import BN from "../../utils/BN";
import { BlockchainNetwork } from "../abstract/BlockchainNetwork";
import { NETWORK_ERROR, NetworkError } from "../NetworkError";
import {
  FetchOrdersParams,
  FetchTradesParams,
  MarketCreateEvent,
  NETWORK,
  SpotMarketVolume,
} from "../types";

import { Api } from "./Api";
import {
  NETWORKS,
  TOKENS_BY_ASSET_ID,
  TOKENS_BY_SYMBOL,
  TOKENS_LIST,
} from "./constants";
import { WalletManager } from "./WalletManager";

export class FuelNetwork extends BlockchainNetwork {
  NETWORK_TYPE = NETWORK.FUEL;

  private providerPromise: Promise<Provider>;

  private walletManager = new WalletManager();
  private api = new Api();

  public network = NETWORKS[0];

  constructor() {
    super();

    makeObservable(this.walletManager);

    this.providerPromise = Provider.create(NETWORKS[0].url);
  }

  getAddress = (): Nullable<string> => {
    return this.walletManager.address;
  };

  getPrivateKey(): Nullable<string> {
    return this.walletManager.privateKey;
  }

  getBalance = async (
    accountAddress: string,
    assetAddress: string,
  ): Promise<string> => {
    return this.walletManager.getBalance(accountAddress, assetAddress);
  };

  getIsExternalWallet = () => false;

  getTokenList = (): Token[] => {
    return TOKENS_LIST;
  };

  getTokenBySymbol = (symbol: string): Token => {
    return TOKENS_BY_SYMBOL[symbol];
  };

  getTokenByAssetId = (assetId: string): Token => {
    return TOKENS_BY_ASSET_ID[assetId.toLowerCase()];
  };

  connectWallet = async (): Promise<void> => { };

  connectWalletByPrivateKey = async (privateKey: string): Promise<void> => {
    await this.walletManager.connectByPrivateKey(
      privateKey,
      await this.providerPromise,
    );
  };

  disconnectWallet = (): void => { };

  addAssetToWallet = async (assetId: string): Promise<void> => { };

  createSpotOrder = async (
    assetAddress: string,
    size: string,
    price: string,
  ): Promise<string> => {
    if (!this.walletManager.wallet) {
      throw new Error("Wallet does not exist");
    }

    const baseToken = this.getTokenByAssetId(assetAddress);
    const quoteToken = this.getTokenBySymbol("USDC");

    return this.api.createSpotOrder(
      baseToken,
      quoteToken,
      size,
      price,
      this.walletManager.wallet,
    );
  };

  cancelSpotOrder = async (orderId: string): Promise<void> => {
    if (!this.walletManager.wallet) {
      throw new Error("Wallet does not exist");
    }

    await this.api.cancelSpotOrder(orderId, this.walletManager.wallet);
  };

  mintToken = async (assetAddress: string, amount?: number): Promise<void> => {
    if (!this.walletManager.wallet) {
      throw new NetworkError(NETWORK_ERROR.UNKNOWN_WALLET);
    }

    await this.api.mintToken(assetAddress, this.walletManager.wallet, amount);
  };

  approve = async (assetAddress: string, amount: string): Promise<void> => { };

  allowance = async (assetAddress: string): Promise<string> => {
    return "9999999999999999";
  };

  depositPerpCollateral = async (
    assetAddress: string,
    amount: string,
  ): Promise<void> => {
    if (!this.walletManager.wallet) {
      throw new NetworkError(NETWORK_ERROR.UNKNOWN_WALLET);
    }

    await this.api.depositPerpCollateral(
      assetAddress,
      amount,
      this.walletManager.wallet,
    );
  };

  withdrawPerpCollateral = async (
    assetAddress: string,
    amount: string,
    oracleUpdateData: string[],
  ): Promise<void> => {
    if (!this.walletManager.wallet) {
      throw new NetworkError(NETWORK_ERROR.UNKNOWN_WALLET);
    }

    await this.api.withdrawPerpCollateral(
      assetAddress,
      amount,
      oracleUpdateData,
      this.walletManager.wallet,
    );
  };

  openPerpOrder = async (
    assetAddress: string,
    amount: string,
    price: string,
    updateData: string[],
  ): Promise<void> => {
    if (!this.walletManager.wallet) {
      throw new NetworkError(NETWORK_ERROR.UNKNOWN_WALLET);
    }

    await this.api.openPerpOrder(
      assetAddress,
      amount,
      price,
      updateData,
      this.walletManager.wallet,
    );
  };

  removePerpOrder = async (assetId: string): Promise<void> => {
    if (!this.walletManager.wallet) {
      throw new NetworkError(NETWORK_ERROR.UNKNOWN_WALLET);
    }

    await this.api.removePerpOrder(assetId, this.walletManager.wallet);
  };

  fetchSpotMarkets = async (limit: number): Promise<MarketCreateEvent[]> => {
    const tokens = [this.getTokenBySymbol("BTC")];
    const providerWallet = await this.getProviderWallet();

    return this.api.fetch.fetchSpotMarkets(limit, tokens, providerWallet);
  };

  fetchSpotMarketPrice = async (baseTokenAddress: string): Promise<BN> => {
    return this.api.fetch.fetchSpotMarketPrice(baseTokenAddress);
  };

  fetchSpotOrders = async (
    params: FetchOrdersParams,
  ): Promise<SpotMarketOrder[]> => {
    const providerWallet = await this.getProviderWallet();

    return this.api.fetch.fetchSpotOrders(params, providerWallet);
  };

  fetchSpotTrades = async (
    params: FetchTradesParams,
  ): Promise<SpotMarketTrade[]> => {
    return this.api.fetch.fetchSpotTrades(params);
  };

  fetchSpotVolume = async (): Promise<SpotMarketVolume> => {
    return this.api.fetch.fetchSpotVolume();
  };

  fetchPerpCollateralBalance = async (
    accountAddress: string,
    assetAddress: string,
  ): Promise<BN> => {
    const providerWallet = await this.getProviderWallet();

    return this.api.fetch.fetchPerpCollateralBalance(
      accountAddress,
      assetAddress,
      providerWallet,
    );
  };

  fetchPerpAllTraderPositions = async (
    accountAddress: string,
  ): Promise<any> => {
    const providerWallet = await this.getProviderWallet();

    return this.api.fetch.fetchPerpAllTraderPositions(
      accountAddress,
      providerWallet,
    );
  };

  fetchPerpIsAllowedCollateral = async (
    assetAddress: string,
  ): Promise<boolean> => {
    const providerWallet = await this.getProviderWallet();

    return this.api.fetch.fetchPerpIsAllowedCollateral(
      assetAddress,
      providerWallet,
    );
  };

  fetchPerpTraderOrders = async (
    accountAddress: string,
    assetAddress: string,
  ) => {
    const providerWallet = await this.getProviderWallet();

    return this.api.fetch.fetchPerpTraderOrders(
      accountAddress,
      assetAddress,
      providerWallet,
    );
  };

  private getProviderWallet = async () => {
    const provider = await this.providerPromise;
    return Wallet.generate({ provider });
  };
}
