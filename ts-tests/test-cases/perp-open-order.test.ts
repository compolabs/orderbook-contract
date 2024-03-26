import { beforeEach, describe, expect, it } from "@jest/globals";
import { EvmPriceServiceConnection } from "@pythnetwork/pyth-evm-js";

import { FuelNetwork } from "../src/blockchain";
import { FAUCET_AMOUNTS } from "../src/constants";
import BN from "../src/utils/BN";

import { PRIVATE_KEY_ALICE, pythURL, TEST_TIMEOUT } from "./constants";

describe("Perp Open Order Tests", () => {
  let fuelNetwork: FuelNetwork;

  let ordersCount: number = 0;
  let lastOrderId: string;

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
    "Deposit USDT",
    async () => {
      const usdc = fuelNetwork.getTokenBySymbol("USDC");
      const address = fuelNetwork.getAddress()!;

      const collateralBalanceBeforeDeposit =
        await fuelNetwork.fetchPerpCollateralBalance(address, usdc.assetId);

      await fuelNetwork.mintToken(usdc.assetId);

      const faucetAmount = FAUCET_AMOUNTS[usdc.symbol];
      const amountToSend = BN.parseUnits(faucetAmount, usdc.decimals);

      const balanceBeforeDepositString = await fuelNetwork.getBalance(
        address,
        usdc.assetId,
      );
      const balanceBeforeDeposit = new BN(balanceBeforeDepositString);

      await fuelNetwork.depositPerpCollateral(
        usdc.assetId,
        amountToSend.toString(),
      );

      const balanceAfterDepositString = await fuelNetwork.getBalance(
        address,
        usdc.assetId,
      );
      const balanceAfterDeposit = new BN(balanceAfterDepositString);

      const collateralBalanceAfterDeposit =
        await fuelNetwork.fetchPerpCollateralBalance(address, usdc.assetId);

      const shouldBalanceBeLess = balanceAfterDeposit.lt(balanceBeforeDeposit);
      const shouldCollateralBalanceBeGreater = collateralBalanceAfterDeposit.gt(
        collateralBalanceBeforeDeposit,
      );

      expect(shouldBalanceBeLess).toBe(true);
      expect(shouldCollateralBalanceBeGreater).toBe(true);
    },
    TEST_TIMEOUT,
  );

  it(
    "Get Orders Before Order",
    async () => {
      const btc = fuelNetwork.getTokenBySymbol("BTC");
      const address = fuelNetwork.getAddress()!;

      const orders = await fuelNetwork.fetchPerpTraderOrders(
        address,
        btc.assetId,
      );

      expect(orders).toBeDefined();
      ordersCount = orders.length;
    },
    TEST_TIMEOUT,
  );

  it(
    "Open Order",
    async () => {
      const btc = fuelNetwork.getTokenBySymbol("BTC");

      const btcPriceFeed = await pythConnection.getLatestPriceFeeds([
        btc.priceFeed,
      ]);

      const updateData = await pythConnection.getPriceFeedsUpdateData([
        btc.priceFeed,
      ]);

      expect(btcPriceFeed).toBeDefined();
      expect(updateData).toBeDefined();

      const btcPrice = btcPriceFeed![0].getPriceUnchecked();
      await fuelNetwork.mintToken(btc.assetId);

      const faucetAmount = FAUCET_AMOUNTS["BTC"];
      const amountToSend = BN.parseUnits(faucetAmount, btc.decimals).toString();

      await fuelNetwork.openPerpOrder(
        btc.assetId,
        amountToSend,
        btcPrice.price,
        updateData,
      );
    },
    TEST_TIMEOUT,
  );

  it(
    "Get Order After Order",
    async () => {
      const btc = fuelNetwork.getTokenBySymbol("BTC");
      const address = fuelNetwork.getAddress()!;

      const orders = await fuelNetwork.fetchPerpTraderOrders(
        address,
        btc.assetId,
      );

      const isOrdersCountIncreased = orders.length > ordersCount;
      expect(isOrdersCountIncreased).toBe(true);
      lastOrderId = orders[0].id;
      ordersCount = orders.length;
    },
    TEST_TIMEOUT,
  );

  it(
    "Close Order",
    async () => {
      expect(lastOrderId).toBeDefined();

      await fuelNetwork.removePerpOrder(lastOrderId);
    },
    TEST_TIMEOUT,
  );

  it(
    "Get Orders After Cancel",
    async () => {
      const btc = fuelNetwork.getTokenBySymbol("BTC");
      const address = fuelNetwork.getAddress()!;

      const orders = await fuelNetwork.fetchPerpTraderOrders(
        address,
        btc.assetId,
      );

      const isOrdersCountDecreased = orders.length < ordersCount;
      expect(isOrdersCountDecreased).toBe(true);
    },
    TEST_TIMEOUT,
  );
});
