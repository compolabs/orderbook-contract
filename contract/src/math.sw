library;
use std::u128::U128;
use i64::I64;

impl u64 {
    pub fn mul_div(self, mul_to: u64, div_to: u64) -> u64 {
        let mul_result = U128::from((0, self)) * U128::from((0, mul_to));
        let div_result = mul_result / U128::from((0, div_to));
        div_result.as_u64().unwrap()
    }

    pub fn mul_div_rounding_up(self, mul_to: u64, div_to: u64) -> u64{
        let mul_result = U128::from((0, self)) * U128::from((0, mul_to));
        let div_to = U128::from((0, div_to));
        let div_result = mul_result / div_to;
        let add = if div_result * div_to < mul_result {1} else {0};
        div_result.as_u64().unwrap() + add
    }

    pub fn as_i64(self) -> I64 {
        I64::from(self)
    }

    pub fn as_neg_i64(self) -> I64 {
        I64::neg_from(self)
    }
}

pub fn max(a: u64, b: u64) -> u64 {
    if a > b { a } else { b }
}

pub fn min(a: u64, b: u64) -> u64 {
    if a < b { a } else { b }
}
