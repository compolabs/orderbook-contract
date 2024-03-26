import { Nullable } from "tsdef";

import { SpotMarketOrder, SpotMarketTrade, Token } from "../../entity";
import { PerpOrder } from "../../entity/PerpOrder";
import BN from "../../utils/BN";
import {
  FetchOrdersParams,
  FetchTradesParams,
  MarketCreateEvent,
  NETWORK,
  SpotMarketVolume,
} from "../types";

export abstract class BlockchainNetwork {
  abstract NETWORK_TYPE: NETWORK;

  abstract getBalance(
    accountAddress: string,
    assetAddress: string,
  ): Promise<string>;
  abstract getAddress(): Nullable<string>;
  abstract getPrivateKey(): Nullable<string>;
  abstract getIsExternalWallet(): boolean;

  // Tokens
  abstract getTokenList(): Token[];
  abstract getTokenBySymbol(symbol: string): Token;
  abstract getTokenByAssetId(assetId: string): Token;

  // Wallet
  abstract connectWallet(): Promise<void>;
  abstract connectWalletByPrivateKey(privateKey: string): Promise<void>;
  abstract disconnectWallet(): void;
  abstract addAssetToWallet(assetId: string): Promise<void>;

  // Api Contract Orderbook
  abstract createSpotOrder(
    assetAddress: string,
    size: string,
    price: string,
  ): Promise<string>;
  abstract cancelSpotOrder(orderId: string): Promise<void>;
  abstract mintToken(assetAddress: string): Promise<void>;
  abstract approve(assetAddress: string, amount: string): Promise<void>;
  abstract allowance(assetAddress: string): Promise<string>;

  // Api Contract Vault
  abstract depositPerpCollateral(
    assetAddress: string,
    amount: string,
  ): Promise<void>;
  abstract withdrawPerpCollateral(
    assetAddress: string,
    amount: string,
    oracleUpdateData: string[],
  ): Promise<void>;
  abstract openPerpOrder(
    assetAddress: string,
    amount: string,
    price: string,
    updateData: string[],
  ): Promise<void>;
  abstract removePerpOrder(orderId: string): Promise<void>;

  // Api Fetch Orderbook
  abstract fetchSpotMarkets(limit: number): Promise<MarketCreateEvent[]>;
  abstract fetchSpotMarketPrice(baseTokenAddress: string): Promise<BN>;
  abstract fetchSpotOrders(
    params: FetchOrdersParams,
  ): Promise<SpotMarketOrder[]>;
  abstract fetchSpotTrades(
    params: FetchTradesParams,
  ): Promise<SpotMarketTrade[]>;
  abstract fetchSpotVolume(): Promise<SpotMarketVolume>;

  // Api Fetch Vault
  abstract fetchPerpCollateralBalance(
    accountAddress: string,
    assetAddress: string,
  ): Promise<BN>;
  abstract fetchPerpAllTraderPositions(accountAddress: string): Promise<any>;
  abstract fetchPerpIsAllowedCollateral(assetAddress: string): Promise<boolean>;
  abstract fetchPerpTraderOrders(
    accountAddress: string,
    assetAddress: string,
  ): Promise<PerpOrder[]>;
}
