import { makeAutoObservable } from "mobx";

import { BlockchainNetworkFactory } from "../blockchain/BlockchainNetworkFactory";
import BN from "../utils/BN";

import { Token } from "./Token";

interface PerpPositionParams {
  baseTokenAddress: string;
  lastTwPremiumGrowthGlobal: BN;
  takerOpenNational: BN;
  takerPositionSize: BN;
}

export class PerpPosition {
  readonly baseToken: Token;
  readonly lastTwPremiumGrowthGlobal: BN;
  readonly takerOpenNational: BN;
  readonly takerPositionSize: BN;

  constructor(params: PerpPositionParams) {
    const bcNetwork = BlockchainNetworkFactory.getInstance().currentInstance!;

    this.baseToken = bcNetwork.getTokenByAssetId(params.baseTokenAddress);

    this.lastTwPremiumGrowthGlobal = params.lastTwPremiumGrowthGlobal;
    this.takerOpenNational = params.takerOpenNational;
    this.takerPositionSize = params.takerPositionSize;

    makeAutoObservable(this);
  }
}
