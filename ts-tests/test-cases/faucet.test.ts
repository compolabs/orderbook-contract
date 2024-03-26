import { beforeEach, describe, expect, it } from "@jest/globals";

import { FuelNetwork } from "../src/blockchain";
import BN from "../src/utils/BN";

import { PRIVATE_KEY_ALICE, TEST_TIMEOUT } from "./constants";

describe("Faucet Tests", () => {
  let fuelNetwork: FuelNetwork;

  beforeEach(async () => {
    fuelNetwork = new FuelNetwork();

    await fuelNetwork.connectWalletByPrivateKey(PRIVATE_KEY_ALICE);
  });

  it(
    "should get balance of USDC",
    async () => {
      const usdc = fuelNetwork.getTokenBySymbol("USDC");

      const address = fuelNetwork.getAddress()!;

      let initialBalanceString = "";
      try {
        initialBalanceString = await fuelNetwork.getBalance(
          address,
          usdc.assetId,
        );
      } catch (error) {
        throw new Error("Retrieving balance should not throw an error.");
      }

      expect(initialBalanceString).toBeDefined();
      expect(initialBalanceString).not.toBe("");
    },
    TEST_TIMEOUT,
  );

  it(
    "should mint a token successfully",
    async () => {
      const usdc = fuelNetwork.getTokenBySymbol("USDC");

      const address = fuelNetwork.getAddress()!;

      const initialBalanceString = await fuelNetwork.getBalance(
        address,
        usdc.assetId,
      );

      const initialBalance = new BN(initialBalanceString).toNumber();

      await fuelNetwork.mintToken(usdc.assetId);

      const newBalanceString = await fuelNetwork.getBalance(
        address,
        usdc.assetId,
      );
      const newBalance = new BN(newBalanceString).toNumber();

      expect(newBalance).toBeGreaterThan(initialBalance);
    },
    TEST_TIMEOUT,
  );
});
