// eslint-disable-next-line @typescript-eslint/no-var-requires
require("dotenv").config();

export const TEST_TIMEOUT = 60_000; // 1min;

export const PRIVATE_KEY_ALICE = process.env.ALICE ?? "";

export const pythURL = "https://hermes.pyth.network";
