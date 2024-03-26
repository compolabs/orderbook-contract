import { Provider, Wallet, WalletUnlocked } from "fuels";
import { makeAutoObservable } from "mobx";
import { Nullable } from "tsdef";

import { NETWORK_ERROR, NetworkError } from "../NetworkError";

export class WalletManager {
  public address: Nullable<string> = null;
  public wallet: Nullable<WalletUnlocked> = null;
  public privateKey: Nullable<string> = null;

  constructor() {
    makeAutoObservable(this);
  }

  connectByPrivateKey = async (
    privateKey: string,
    provider: Provider,
  ): Promise<void> => {
    const wallet = Wallet.fromPrivateKey(privateKey, provider);

    const address = wallet.address.toString();
    this.address = address;
    this.privateKey = privateKey;
    this.wallet = wallet;
  };

  getBalance = async (address: string, assetId: string) => {
    if (!this.wallet) {
      throw new NetworkError(NETWORK_ERROR.UNKNOWN_WALLET);
    }

    const balance = await this.wallet.getBalance(assetId);
    return balance.toString();
  };
}
