import { beforeEach, describe, expect, it } from "@jest/globals";
import { EvmPriceServiceConnection } from "@pythnetwork/pyth-evm-js";

import { FuelNetwork } from "../src/blockchain";
import { FAUCET_AMOUNTS } from "../src/constants";
import BN from "../src/utils/BN";

import { PRIVATE_KEY_ALICE, pythURL, TEST_TIMEOUT } from "./constants";

describe("Perp Deposit \\ Withdraw Tests", () => {
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
    "Check ETH balance",
    async () => {
      const eth = fuelNetwork.getTokenBySymbol("ETH");
      const address = fuelNetwork.getAddress()!;

      const balance = await fuelNetwork.getBalance(address, eth.assetId);

      const isEnoughEth = new BN(balance).gt(BN.ZERO);

      expect(isEnoughEth).toBe(true);
    },
    TEST_TIMEOUT,
  );

  it(
    "Check USDC collateral",
    async () => {
      const usdc = fuelNetwork.getTokenBySymbol("USDC");

      const isUSDCAllowed = await fuelNetwork.fetchPerpIsAllowedCollateral(
        usdc.assetId,
      );

      expect(isUSDCAllowed).toBe(true);
    },
    TEST_TIMEOUT,
  );

  it(
    "Deposit USDC",
    async () => {
      const usdc = fuelNetwork.getTokenBySymbol("USDC");
      const address = fuelNetwork.getAddress()!;

      const collateralBalanceBeforeDeposit =
        await fuelNetwork.fetchPerpCollateralBalance(address, usdc.assetId);

      await fuelNetwork.mintToken(usdc.assetId);

      const faucetAmount = FAUCET_AMOUNTS["USDC"];
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
    "Withdraw USDC",
    async () => {
      const usdc = fuelNetwork.getTokenBySymbol("USDC");
      const address = fuelNetwork.getAddress()!;

      const collateralBalanceBeforeWithdraw =
        await fuelNetwork.fetchPerpCollateralBalance(address, usdc.assetId);

      const faucetAmount = FAUCET_AMOUNTS["USDC"];
      const amountToWithdraw = BN.parseUnits(faucetAmount, usdc.decimals);

      const balanceBeforeWithdrawString = await fuelNetwork.getBalance(
        address,
        usdc.assetId,
      );
      const balanceBeforeWithdraw = new BN(balanceBeforeWithdrawString);

      const updateData = await pythConnection.getPriceFeedsUpdateData([
        usdc.priceFeed,
      ]);

      await fuelNetwork.withdrawPerpCollateral(
        usdc.assetId,
        amountToWithdraw.toString(),
        updateData,
      );

      const balanceAfterWithdrawString = await fuelNetwork.getBalance(
        address,
        usdc.assetId,
      );
      const balanceAfterWithdraw = new BN(balanceAfterWithdrawString);

      const collateralBalanceAfterWithdraw =
        await fuelNetwork.fetchPerpCollateralBalance(address, usdc.assetId);

      const shouldBalanceBeGreater = balanceAfterWithdraw.gt(
        balanceBeforeWithdraw,
      );
      const shouldCollateralBalanceBeLess = collateralBalanceAfterWithdraw.lt(
        collateralBalanceBeforeWithdraw,
      );

      expect(shouldBalanceBeGreater).toBe(true);
      expect(shouldCollateralBalanceBeLess).toBe(true);
    },
    TEST_TIMEOUT,
  );
});
