import { beforeEach, describe, expect, it } from "@jest/globals";

import { FuelNetwork } from "../src/blockchain";

import { PRIVATE_KEY_ALICE, TEST_TIMEOUT } from "./constants";
import { getDecodedLogs } from "fuels";

const MOCK_BTC_AMOUNT = "100000";
const MOCK_BTC_PRICE = "67000000000000";

describe("Spot Test", () => {
  let fuelNetwork: FuelNetwork;
  let orderId: string;
  let ordersAmount: number;

  beforeEach(async () => {
    fuelNetwork = new FuelNetwork();

    await fuelNetwork.connectWalletByPrivateKey(PRIVATE_KEY_ALICE);
  });

  it(
    "should decode",
    async () => {

      getDecodedLogs()
    },
    TEST_TIMEOUT,
  );


});
