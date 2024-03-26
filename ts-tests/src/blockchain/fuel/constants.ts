import { Token } from "../../entity";

import TOKENS_JSON from "./tokens.json";

export const CONTRACT_ADDRESSES = {
  spotMarket:
    "0x09888861682fb900a7ea36806fb66074ffc3a69fc4f05ed17730b0bc417a0fe0",
  tokenFactory:
    "0x6bd9643c9279204b474a778dea7f923226060cb94a4c61c5aae015cf96b5aad2",
  vault: "0xba7e8d20c93fb776a369ddba3b76c0716f7fd9a71ee99afd4a45a2c2c5cdba34",
  accountBalance:
    "0x5dcab75350f40f3f77ad85dea4b8d4e191b00a9e4e0373405772f90e97e38258",
  clearingHouse:
    "0x4a023ae9417e2be14cf8548857046cd44759f1716dc9fee96f472f6862d2a0ac",
  perpMarket:
    "0x3fc41126bb79d32f61cddce43b96c6db28d37b6ed592e42d5e64d78bdf5e0e6c",
  pyth: "0xe67badf5987def9b0cd8661cb2d8ae4290b60b189be7b8c3d12c064610095888",
  proxy: "0xc26bd3ee8bbc60584c3a10df2aacb924a374e4ba4e5014248b78e829e558b29b",
};

export interface Network {
  name: string;
  url: string;
}

export const NETWORKS: Network[] = [
  {
    name: "Fuel",
    url: "https://beta-5.fuel.network/graphql",
  },
];

export const EXPLORER_URL =
  "https://fuellabs.github.io/block-explorer-v2/beta-5";

export const TOKENS_LIST: Token[] = Object.values(TOKENS_JSON).map(
  ({ name, symbol, decimals, assetId, priceFeed }) => {
    return new Token({
      name,
      symbol,
      decimals,
      assetId,
      logo: "",
      priceFeed,
    });
  },
);

export const TOKENS_BY_SYMBOL: Record<string, Token> = TOKENS_LIST.reduce(
  (acc, t) => ({ ...acc, [t.symbol]: t }),
  {},
);

export const TOKENS_BY_ASSET_ID: Record<string, Token> = TOKENS_LIST.reduce(
  (acc, t) => ({ ...acc, [t.assetId.toLowerCase()]: t }),
  {},
);

export const GAS_LIMIT = 20000000;
