import { BETA_TOKENS } from "@compolabs/spark-ts-sdk";

// eslint-disable-next-line @typescript-eslint/no-var-requires
require("dotenv").config();

export const TEST_TIMEOUT = 60_000; // 1min;

export const PRIVATE_KEY_ALICE = process.env.ALICE ?? "";

export const pythURL = "https://hermes.pyth.network";

interface Asset {
  address: string;
  symbol: string;
  decimals: number;
  priceFeed: string;
}

export const TOKENS_LIST: Asset[] = Object.values(BETA_TOKENS).map(
  ({ decimals, assetId, symbol, priceFeed }) => ({
    address: assetId,
    symbol,
    decimals,
    priceFeed,
  }),
);

export const TOKENS_BY_SYMBOL: Record<string, Asset> = TOKENS_LIST.reduce(
  (acc, t) => ({ ...acc, [t.symbol]: t }),
  {},
);

export const TOKENS_BY_ASSET_ID: Record<string, Asset> = TOKENS_LIST.reduce(
  (acc, t) => ({ ...acc, [t.address.toLowerCase()]: t }),
  {},
);

export const FAUCET_AMOUNTS = {
  ETH: "0.001",
  USDC: "3000",
  BTC: "0.01",
  UNI: "50",
};
