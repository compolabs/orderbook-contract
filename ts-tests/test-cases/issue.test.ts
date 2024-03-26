import { beforeEach, describe, it } from "@jest/globals";
import { EvmPriceServiceConnection } from "@pythnetwork/pyth-evm-js";

import { FuelNetwork } from "../src/blockchain";
import BN from "../src/utils/BN";

import { PRIVATE_KEY_ALICE, pythURL, TEST_TIMEOUT } from "./constants";

describe("Issue with Open Order", () => {
  let fuelNetwork: FuelNetwork;

  const pythConnection = new EvmPriceServiceConnection(pythURL, {
    logger: {
      error: console.error,
      warn: console.warn,
      info: () => undefined,
      debug: () => undefined,
      trace: () => undefined,
    },
  });

  beforeEach(async () => {
    fuelNetwork = new FuelNetwork();

    await fuelNetwork.connectWalletByPrivateKey(PRIVATE_KEY_ALICE);
  });

  it(
    "Test with issue",
    async () => {
      const usdc = fuelNetwork.getTokenBySymbol("USDC");
      const address = fuelNetwork.getAddress()!;

      console.log("USDC", usdc);
      console.log("User Address", address);

      await fuelNetwork.mintToken(usdc.assetId);

      const faucetUsdcAmount = 21000;
      const amountUsdcToSend = BN.parseUnits(faucetUsdcAmount, usdc.decimals);

      console.log("Amount of USDC", amountUsdcToSend.toString());

      await fuelNetwork.mintToken(usdc.assetId, amountUsdcToSend.toNumber());

      await fuelNetwork.depositPerpCollateral(
        usdc.assetId,
        amountUsdcToSend.toString(),
      );

      console.log("Deposit done");

      const btc = fuelNetwork.getTokenBySymbol("BTC");

      const btcPriceFeed = await pythConnection.getLatestPriceFeeds([
        btc.priceFeed,
      ]);

      const updateData = await pythConnection.getPriceFeedsUpdateData([
        btc.priceFeed,
      ]);

      const btcPrice = btcPriceFeed![0].getPriceUnchecked();

      const faucetBtcAmount = 0.01;
      const amountBtcToSend = BN.parseUnits(
        faucetBtcAmount,
        btc.decimals,
      ).toString();

      console.log("BTC", btc);
      console.log("BTC Amount", amountBtcToSend.toString());
      console.log("Update Data", updateData);

      await fuelNetwork.openPerpOrder(
        btc.assetId,
        amountBtcToSend,
        btcPrice.price,
        updateData,
      );

      console.log("Open Order done");
    },
    TEST_TIMEOUT,
  );
});
