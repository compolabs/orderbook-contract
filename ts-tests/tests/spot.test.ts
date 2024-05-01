import Spark, {
  BETA_CONTRACT_ADDRESSES,
  BETA_INDEXER_URL,
  BETA_NETWORK,
} from "@compolabs/spark-ts-sdk";
import { beforeEach, describe, expect, it } from "@jest/globals";
import { Provider, sleep, Wallet, WalletUnlocked } from "fuels";

import {
  PRIVATE_KEY_ALICE,
  TEST_TIMEOUT,
  TOKENS_BY_SYMBOL,
} from "../src/constants";

const MOCK_BTC_AMOUNT = "10000";
const MOCK_BTC_PRICE = "60000000000000";

describe("Spot Test", () => {
  let wallet: WalletUnlocked;
  let spark: Spark;

  let orderId: string;
  let ordersAmount: number;

  beforeEach(async () => {
    const provider = await Provider.create(BETA_NETWORK.url);
    wallet = Wallet.fromPrivateKey(PRIVATE_KEY_ALICE, provider);

    spark = new Spark({
      networkUrl: BETA_NETWORK.url,
      contractAddresses: BETA_CONTRACT_ADDRESSES,
      indexerApiUrl: BETA_INDEXER_URL,
      wallet,
    });
  });

  it(
    "should create order",
    async () => {
      const btc = TOKENS_BY_SYMBOL["BTC"];
      const usdc = TOKENS_BY_SYMBOL["USDC"];

      let hash = "";
      try {
        hash = await spark.createSpotOrder(
          btc,
          usdc,
          MOCK_BTC_AMOUNT,
          MOCK_BTC_PRICE,
        );
      } catch (error) {
        throw new Error(`Create order should not throw an error: ${error}`);
      }

      expect(hash).toBeDefined();
      expect(hash).not.toBe("");
    },
    TEST_TIMEOUT,
  );

  it(
    "should fetch orders",
    async () => {
      const btc = TOKENS_BY_SYMBOL["BTC"];
      const address = wallet.address.toAddress();

      let orders;
      try {
        orders = await spark.fetchSpotOrders({
          baseToken: btc.address,
          limit: 100,
          trader: address,
          isActive: true,
        });
      } catch (error) {
        throw new Error(`Fetch orders should not throw an error: ${error}`);
      }

      expect(orders).toBeDefined();
      expect(orders).not.toHaveLength(0);

      orderId = orders[0].id;
      ordersAmount = orders.length;
    },
    TEST_TIMEOUT,
  );

  it(
    "should cancel order",
    async () => {
      try {
        await spark.cancelSpotOrder(orderId);
      } catch (error) {
        throw new Error(`Cancel order should not throw an error: ${error}`);
      }
    },
    TEST_TIMEOUT,
  );

  it(
    "should remove order from fetchOrders",
    async () => {
      const btc = TOKENS_BY_SYMBOL["BTC"];
      const address = wallet.address.toAddress();

      let orders;
      try {
        await sleep(5000);
        orders = await spark.fetchSpotOrders({
          baseToken: btc.address,
          limit: 100,
          trader: address,
          isActive: true,
        });
      } catch (error) {
        throw new Error(`Fetch orders should not throw an error: ${error}`);
      }

      expect(orders).toBeDefined();
      expect(orders.length).toBeLessThan(ordersAmount);
    },
    TEST_TIMEOUT,
  );
});
