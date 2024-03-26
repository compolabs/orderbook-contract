import { makeAutoObservable } from "mobx";

import { BlockchainNetworkFactory } from "../blockchain/BlockchainNetworkFactory";
import BN from "../utils/BN";

import { Token } from "./Token";

interface PerpOrderParams {
  id: string;
  baseSize: BN;
  baseTokenAddress: string;
  orderPrice: BN;
  trader: string;
}

export class PerpOrder {
  readonly baseToken: Token;
  readonly id: string;
  readonly baseSize: BN;
  readonly orderPrice: BN;
  readonly trader: string;

  constructor(params: PerpOrderParams) {
    const bcNetwork = BlockchainNetworkFactory.getInstance().currentInstance!;

    this.baseToken = bcNetwork.getTokenByAssetId(params.baseTokenAddress);

    this.id = params.id;
    this.baseSize = params.baseSize;
    this.orderPrice = params.orderPrice;
    this.trader = params.trader;

    makeAutoObservable(this);
  }
}
