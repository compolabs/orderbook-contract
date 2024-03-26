import { BN as FuelBN } from "fuels";

import BN from "../../utils/BN";

export const convertI64ToBn = (input: {
  value: FuelBN;
  negative: boolean;
}): BN => {
  return new BN(input.value.toString()).multipliedBy(input.negative ? -1 : 1);
};
