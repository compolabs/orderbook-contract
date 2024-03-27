import {beforeEach, describe, expect, it} from "@jest/globals";

import {FuelNetwork} from "../src/blockchain";

import {PRIVATE_KEY_ALICE, TEST_TIMEOUT} from "./constants";
import {getDecodedLogs} from "fuels";
import {OrderbookAbi__factory} from "../src/blockchain/fuel/types/orderbook";
import {CONTRACT_ADDRESSES} from "../src/blockchain/fuel/constants";
import indexerData from "./indexer_data.json";
import {ReceiptLogData} from "@fuel-ts/transactions/dist/coders/receipt";
import {BN} from "@fuel-ts/math";

describe("Spot Test", () => {
    let fuelNetwork: FuelNetwork;

    beforeEach(async () => {
        fuelNetwork = new FuelNetwork();
        await fuelNetwork.connectWalletByPrivateKey(PRIVATE_KEY_ALICE);
    });

    it(
        "should decode",
        async () => {
            const orderbookFactory = OrderbookAbi__factory.connect(
                CONTRACT_ADDRESSES.spotMarket,
                await fuelNetwork.getProviderWallet(),
            );
            const receipts = (indexerData as any).data[0].receipts.filter(({receipt_type}: any) => receipt_type == 6).map((receipt: any) => ({
                type: receipt.receipt_type,
                id: receipt.tx_id,
                val0: new BN(receipt.ra),
                val1: new BN(receipt.rb),
                ptr: new BN(receipt.ptr),
                len: new BN(receipt.len),
                digest: receipt.digest,
                pc: new BN(receipt.pc),
                is: new BN(receipt.is),
            } as ReceiptLogData))

            const logs = getDecodedLogs(receipts, orderbookFactory.interface)
            console.log(logs)
        },
        TEST_TIMEOUT,
    );
});

/*
type - receipt_type
id - tx_id
val0 - ra
val1 - rb
ptr - ?
len - ?
digest - ?
pc - pc
is - is
* */

/*   {
      tx_id: '0xb12e20dd9483160661e7a988b88125b4fc691181e7b1fd19b825bba31cdd2ed9',
      pc: 30464,
      is: 13160,
      to: null,
      to_address: null,
      amount: null,
      param1: null,
      param2: null,
      val: null,
      reason: null,
      ra: 0,
      rb: 9,
      rc: null,
      rd: null,
      receipt_type: 6,
      result: null,
      data: '0x000000000000000f',
      sender: null,
      recipient: null,
      contract_id: null
    }

*
* */
