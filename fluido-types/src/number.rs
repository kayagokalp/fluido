use serde::{Deserialize, Serialize};
use std::{
    cmp::max,
    num::ParseFloatError,
    ops::{Add, Div, Mul, Sub},
    str::FromStr,
};

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub struct LimitedFloat {
    pub wrapped: i64,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Serialize, Deserialize)]
pub struct Frac {
    numerator: i32,
    power: i32,
}

impl Frac {
    fn new(numerator: i32, power: i32) -> Self {
        Self { numerator, power }
    }
}

impl Add for Frac {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        if self.power == other.power {
            // If powers are equal, simply add the numerators
            Self::new(self.numerator + other.numerator, self.power)
        } else {
            // If powers are different, align them to the common power
            let common_power = max(self.power, other.power);
            let numerator1 = self.numerator << (common_power - self.power);
            let numerator2 = other.numerator << (common_power - other.power);
            Self::new(numerator1 + numerator2, common_power)
        }
    }
}

impl Sub for Frac {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        if self.power == other.power {
            // If powers are equal, simply subtract the numerators
            Self::new(self.numerator - other.numerator, self.power)
        } else {
            // If powers are different, align them to the common power
            let common_power = max(self.power, other.power);
            let numerator1 = self.numerator << (common_power - self.power);
            let numerator2 = other.numerator << (common_power - other.power);
            Self::new(numerator1 - numerator2, common_power)
        }
    }
}

impl Mul for Frac {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        // Multiply the numerators and add the powers
        Self::new(self.numerator * other.numerator, self.power + other.power)
    }
}

impl Div for Frac {
    type Output = Self;

    fn div(self, other: Self) -> Self {
        // Divide the numerators and subtract the powers
        Self::new(self.numerator / other.numerator, self.power - other.power)
    }
}

impl LimitedFloat {
    pub fn valid(&self) -> bool {
        self.wrapped >= 0 && self.wrapped as f64 <= 1.0f64 / Self::EPSILON
    }

    pub const EPSILON: f64 = 0.0001;
}

impl Sub for LimitedFloat {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        let self_val = self.wrapped;
        let rhs_val = rhs.wrapped;
        let val = self_val - rhs_val;

        Self { wrapped: val }
    }
}

impl Add for LimitedFloat {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let self_val = self.wrapped;
        let rhs_val = rhs.wrapped;
        let val = self_val + rhs_val;

        Self { wrapped: val }
    }
}

impl Div for LimitedFloat {
    type Output = LimitedFloat;

    fn div(self, rhs: Self) -> Self::Output {
        let self_val: f64 = self.into();
        let rhs_val: f64 = rhs.into();

        let res = self_val / rhs_val;
        LimitedFloat::from(res)
    }
}

impl Mul for LimitedFloat {
    type Output = LimitedFloat;

    fn mul(self, rhs: Self) -> Self::Output {
        let self_val: f64 = self.into();
        let rhs_val: f64 = rhs.into();

        let res = self_val * rhs_val;
        LimitedFloat::from(res)
    }
}

impl From<LimitedFloat> for f64 {
    fn from(value: LimitedFloat) -> Self {
        let epsilon_corrected = value.wrapped as f64 * LimitedFloat::EPSILON;
        let scale = 1f64 / Self::EPSILON;
        (epsilon_corrected * scale).trunc() / scale
    }
}

impl From<f64> for LimitedFloat {
    fn from(value: f64) -> Self {
        Self {
            wrapped: (value / Self::EPSILON).round() as i64,
        }
    }
}

impl FromStr for LimitedFloat {
    type Err = ParseFloatError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let f64_val = s.parse::<f64>()?;
        let epsilon_corrected = (f64_val / Self::EPSILON).round() as i64;

        Ok(Self {
            wrapped: epsilon_corrected,
        })
    }
}

impl std::fmt::Display for LimitedFloat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let epsilon_corrected = self.wrapped as f64 * Self::EPSILON;
        let scale = 1f64 / Self::EPSILON;
        let truncated = (epsilon_corrected * scale).trunc() / scale;

        if truncated.fract() == 0.0 {
            write!(f, "{}.0", truncated)
        } else {
            write!(f, "{}", truncated)
        }
    }
}

#[cfg(test)]
mod tests {
    use serde_test::{assert_tokens, Token};

    use crate::number::Frac;

    use super::LimitedFloat;

    #[test]
    fn test_lf_ser_de() {
        let num_a = 0.5;
        let num_a_wrapped = num_a / LimitedFloat::EPSILON;
        let num_a_wrapped = num_a_wrapped as i64;
        let conc_a = LimitedFloat::from(num_a);

        assert_tokens(
            &conc_a,
            &[
                Token::Struct {
                    name: "LimitedFloat",
                    len: 1,
                },
                Token::Str("wrapped"),
                Token::I64(num_a_wrapped),
                Token::StructEnd,
            ],
        );
    }

    #[test]
    fn test_lf_valid() {
        let num_a = 0.5;
        let lf = LimitedFloat::from(num_a);

        assert!(lf.valid())
    }

    #[test]
    fn test_lf_not_valid() {
        let lf = LimitedFloat { wrapped: -100 };
        let lf2 = LimitedFloat { wrapped: 100000000 };

        assert!(!lf.valid());
        assert!(!lf2.valid())
    }

    #[test]
    fn test_lf_precision() {
        let num_a = 0.00005;
        let num_b = 0.00009;

        let conc_a = LimitedFloat::from(num_a);
        let conc_b = LimitedFloat::from(num_b);

        assert_eq!(conc_a, conc_b)
    }

    #[test]
    fn test_lf_addition() {
        let num_a: LimitedFloat = 0.01f64.into();
        let num_b: LimitedFloat = 0.01f64.into();

        let num_c: LimitedFloat = 0.9f64.into();
        let num_d: LimitedFloat = 0.1f64.into();

        let expected: LimitedFloat = 0.02f64.into();
        let expected_2: LimitedFloat = 1.0f64.into();
        let sum = num_a + num_b;
        let sum2 = num_c + num_d;
        assert_eq!(sum, expected);
        assert_eq!(sum2, expected_2);
    }

    #[test]
    fn test_lf_sub() {
        let num_a: LimitedFloat = 0.01f64.into();
        let num_b: LimitedFloat = 0.01f64.into();

        let expected: LimitedFloat = 0f64.into();
        let diff = num_a - num_b;
        assert_eq!(diff, expected)
    }

    #[test]
    fn test_lf_div() {
        let num_a: LimitedFloat = 1.0f64.into();
        let num_b: LimitedFloat = 2.0f64.into();

        let expected: LimitedFloat = 0.5f64.into();
        let diff = num_a / num_b;
        assert_eq!(diff, expected)
    }

    #[test]
    fn test_lf_mul() {
        let num_a: LimitedFloat = 0.5f64.into();
        let num_b: LimitedFloat = 2.0f64.into();

        let expected: LimitedFloat = 1.0f64.into();
        let diff = num_a * num_b;
        assert_eq!(diff, expected)
    }

    #[test]
    fn test_lf_display() {
        let num_a: LimitedFloat = 0.01f64.into();
        let expected = "0.01";
        let num_a_str = format!("{num_a}");
        assert_eq!(num_a_str, expected);
        let num_b: LimitedFloat = 0f64.into();
        let expected = "0.0";
        let num_b_str = format!("{num_b}");
        assert_eq!(num_b_str, expected);
    }

    #[test]
    fn test_frac_add_same_power() {
        let a = Frac::new(1, 2);
        let b = Frac::new(1, 2);
        let result = a + b;
        assert_eq!(result, Frac::new(2, 2)); // 2/4 = 1/2^2
    }

    #[test]
    fn test_frac_add_different_power() {
        let a = Frac::new(1, 2);
        let b = Frac::new(1, 3);
        let result = a + b;
        assert_eq!(result, Frac::new(3, 3)); // 1/4 + 1/8 = 3/8 = 3/2^3
    }

    #[test]
    fn test_frac_sub_same_power() {
        let a = Frac::new(2, 2);
        let b = Frac::new(1, 2);
        let result = a - b;
        assert_eq!(result, Frac::new(1, 2)); // 2/4 - 1/4 = 1/4 = 1/2^2
    }

    #[test]
    fn test_frac_sub_different_power() {
        let a = Frac::new(1, 2);
        let b = Frac::new(1, 3);
        let result = a - b;
        assert_eq!(result, Frac::new(1, 3)); // 1/4 - 1/8 = 1/8 = 1/2^3
    }

    #[test]
    fn test_frac_mul() {
        let a = Frac::new(1, 2);
        let b = Frac::new(1, 3);
        let result = a * b;
        assert_eq!(result, Frac::new(1, 5)); // 1/4 * 1/8 = 1/32 = 1/2^5
    }

    #[test]
    fn test_frac_div() {
        let a = Frac::new(1, 2);
        let b = Frac::new(1, 3);
        let result = a / b;
        assert_eq!(result, Frac::new(1, -1)); // 1/4 / 1/8 = 2 = 1/2^-1
    }

    #[test]
    fn test_frac_ser_de() {
        let a = Frac::new(1, 2);

        assert_tokens(
            &a,
            &[
                Token::Struct {
                    name: "Frac",
                    len: 2,
                },
                Token::Str("numerator"),
                Token::I32(1),
                Token::Str("power"),
                Token::I32(2),
                Token::StructEnd,
            ],
        );
    }
}
