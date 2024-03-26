import { makeAutoObservable } from "mobx";

import { BlockchainNetworkFactory } from "../blockchain/BlockchainNetworkFactory";
import { DEFAULT_DECIMALS } from "../constants";
import BN from "../utils/BN";

import { Token } from "./Token";

interface PerpMarketParams {
  baseTokenAddress: string;
  quoteTokenAddress: string;
  imRatio: BN;
  mmRatio: BN;
  status: "Opened" | "Paused" | "Closed";
  pausedIndexPrice?: BN;
  pausedTimestamp?: number;
  closedPrice?: BN;
}

export class PerpMarket {
  readonly baseToken: Token;
  readonly quoteToken: Token;

  readonly imRatio: BN;
  readonly mmRatio: BN;
  readonly status: "Opened" | "Paused" | "Closed";
  readonly pausedIndexPrice?: BN;
  readonly pausedTimestamp?: number;
  readonly closedPrice?: BN;

  price: BN = BN.ZERO;
  setPrice = (price: BN) => (this.price = price);

  constructor(params: PerpMarketParams) {
    const bcNetwork = BlockchainNetworkFactory.getInstance().currentInstance!;

    this.baseToken = bcNetwork.getTokenByAssetId(params.baseTokenAddress);
    this.quoteToken = bcNetwork.getTokenByAssetId(params.quoteTokenAddress);

    this.imRatio = params.imRatio;
    this.mmRatio = params.mmRatio;
    this.status = params.status;
    this.pausedIndexPrice = params.pausedIndexPrice;
    this.pausedTimestamp = params.pausedTimestamp;
    this.closedPrice = params.closedPrice;

    makeAutoObservable(this);
  }

  get symbol(): string {
    return `${this.baseToken.symbol}-${this.quoteToken.symbol}`;
  }

  get priceUnits(): BN {
    return BN.formatUnits(this.price, DEFAULT_DECIMALS);
  }
}
