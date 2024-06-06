use serde::{Deserialize, Serialize};
use std::{
    num::ParseFloatError,
    ops::{Add, Div, Mul, Sub},
    str::FromStr,
};

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub struct LimitedFloat {
    pub wrapped: i64,
}

pub type Concentration = LimitedFloat;
pub type Volume = LimitedFloat;

impl LimitedFloat {
    pub fn new(wrapped: i64) -> Self {
        Self { wrapped }
    }

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
    fn from(value: Concentration) -> Self {
        let epsilon_corrected = value.wrapped as f64 * Concentration::EPSILON;
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
    use super::LimitedFloat;

    #[test]
    fn test_precision() {
        let num_a = 0.00005;
        let num_b = 0.00009;

        let conc_a = LimitedFloat::from(num_a);
        let conc_b = LimitedFloat::from(num_b);

        assert_eq!(conc_a, conc_b)
    }

    #[test]
    fn test_addition() {
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
    fn test_sub() {
        let num_a: LimitedFloat = 0.01f64.into();
        let num_b: LimitedFloat = 0.01f64.into();

        let expected: LimitedFloat = 0f64.into();
        let diff = num_a - num_b;
        assert_eq!(diff, expected)
    }

    #[test]
    fn test_div() {
        let num_a: LimitedFloat = 1.0f64.into();
        let num_b: LimitedFloat = 2.0f64.into();

        let expected: LimitedFloat = 0.5f64.into();
        let diff = num_a / num_b;
        assert_eq!(diff, expected)
    }

    #[test]
    fn test_mul() {
        let num_a: LimitedFloat = 0.5f64.into();
        let num_b: LimitedFloat = 2.0f64.into();

        let expected: LimitedFloat = 1.0f64.into();
        let diff = num_a * num_b;
        assert_eq!(diff, expected)
    }

    #[test]
    fn test_display() {
        let num_a: LimitedFloat = 0.01f64.into();
        let expected = "0.01";
        let num_a_str = format!("{num_a}");
        assert_eq!(num_a_str, expected);
        let num_b: LimitedFloat = 0f64.into();
        let expected = "0.0";
        let num_b_str = format!("{num_b}");
        assert_eq!(num_b_str, expected);
    }
}
