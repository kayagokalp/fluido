use fraction::Fraction;
use serde::{Deserialize, Serialize};
use std::{
    fmt::{Debug, Display},
    ops::{Add, Div, Mul, Sub},
    str::FromStr,
};

pub trait SaturationNumber:
    Clone + From<f64> + Into<f64> + Display + Add + Sub + Mul + Div + Debug
{
    fn valid(&self) -> bool;
    fn parse(str: &str) -> anyhow::Result<Self>;
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub struct LimitedFloat {
    pub wrapped: i64,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Serialize, Deserialize, PartialOrd, Ord, Hash)]
pub struct Frac {
    fraction: Fraction,
}

impl SaturationNumber for Frac {
    fn valid(&self) -> bool {
        let f64_val: f64 = self.into();
        f64_val >= 0.0 && f64_val < 1.0
    }

    fn parse(str: &str) -> anyhow::Result<Self> {
        Self::from_str(str)
    }
}

impl Add for Frac {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        let new_frac = self.fraction + other.fraction;
        Self { fraction: new_frac }
    }
}

impl Sub for Frac {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        let new_frac = self.fraction - other.fraction;
        Self { fraction: new_frac }
    }
}

impl Mul for Frac {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        // Multiply the numerators and add the powers
        let new_frac = self.fraction * other.fraction;
        Self { fraction: new_frac }
    }
}

impl Div for Frac {
    type Output = Self;

    fn div(self, other: Self) -> Self {
        // Divide the numerators and subtract the powers
        let new_frac = self.fraction / other.fraction;
        Self { fraction: new_frac }
    }
}

// TODO: differentiate this from LimitedFloat.
impl FromStr for Frac {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lf = LimitedFloat::from_str(s)?;
        let f64_val: f64 = lf.into();
        Ok(Self::from(f64_val))
    }
}

// TODO: differentiate this from LimitedFloat.
impl Display for Frac {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let f64_val: f64 = self.into();
        let lf = LimitedFloat::from(f64_val);
        write!(f, "{}", lf)
    }
}

/// Blanket implementation for f64 <-> Frac conversion.
impl<T> From<&T> for Frac
where
    T: Into<Frac>,
{
    fn from(value: &T) -> Self {
        // TODO: consider making Frac a copy type.
        value.into()
    }
}

impl From<&Frac> for f64 {
    fn from(value: &Frac) -> Self {
        let val = value.clone();
        val.into()
    }
}

impl From<f64> for Frac {
    fn from(value: f64) -> Self {
        let fraction = Fraction::from(value);
        Self { fraction }
    }
}

impl From<Frac> for f64 {
    fn from(value: Frac) -> Self {
        value.fraction.try_into().unwrap()
    }
}

impl SaturationNumber for LimitedFloat {
    fn valid(&self) -> bool {
        self.wrapped >= 0 && self.wrapped as f64 <= 1.0f64 / Self::EPSILON
    }

    fn parse(str: &str) -> anyhow::Result<Self> {
        Self::from_str(str)
    }
}

impl LimitedFloat {
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
    type Err = anyhow::Error;

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
    use crate::number::{Frac, SaturationNumber};
    use fraction::Fraction;
    use serde_test::{assert_tokens, Token};

    /// Frac::new impl for easy testing.
    impl Frac {
        fn new(num1: u16, num2: u16) -> Self {
            let fraction = Fraction::new(num1, num2);
            Self { fraction }
        }
    }

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
        assert_eq!(result, Frac::new(2, 2)); // 2/2
    }

    #[test]
    fn test_frac_add_different_power() {
        let a = Frac::new(1, 2);
        let b = Frac::new(1, 3);
        let result = a + b;
        assert_eq!(result, Frac::new(5, 6)); // 1/2 + 1/3 = 5/6
    }

    #[test]
    fn test_frac_sub_same_power() {
        let a = Frac::new(2, 2);
        let b = Frac::new(1, 2);
        let result = a - b;
        assert_eq!(result, Frac::new(1, 2)); // 2/2 - 1/2 = 1/2
    }

    #[test]
    fn test_frac_sub_different_power() {
        let a = Frac::new(1, 2);
        let b = Frac::new(1, 3);
        let result = a - b;
        assert_eq!(result, Frac::new(1, 6)); // 1/2 - 1/3 = 1/6
    }

    #[test]
    fn test_frac_mul() {
        let a = Frac::new(1, 2);
        let b = Frac::new(1, 3);
        let result = a * b;
        assert_eq!(result, Frac::new(1, 6)); // 1/2 * 1/3 = 1/6
    }

    #[test]
    fn test_frac_div() {
        let a = Frac::new(1, 2);
        let b = Frac::new(1, 3);
        let result = a / b;
        assert_eq!(result, Frac::new(3, 2)); // 1/2 / 1/3 = 3/2
    }

    #[test]
    fn test_frac_ser_de() {
        let a = Frac::new(1, 2);

        assert_tokens(
            &a,
            &[
                Token::Struct {
                    name: "Frac",
                    len: 1,
                },
                Token::Str("fraction"),
                Token::TupleVariant {
                    name: "GenericFraction",
                    variant: "Rational",
                    len: 2,
                },
                Token::UnitVariant {
                    name: "Sign",
                    variant: "Plus",
                },
                Token::Tuple { len: 2 },
                Token::U64(1),
                Token::U64(2),
                Token::TupleEnd,
                Token::TupleVariantEnd,
                Token::StructEnd,
            ],
        );
    }

    #[test]
    fn frac_from_str() {
        let frac_str = "0.5";
        let frac = frac_str.parse::<Frac>().unwrap();
        let expected_num = 1;
        let expected_pow = 2;
        let expected_frac = Frac::new(expected_num, expected_pow);

        assert_eq!(frac, expected_frac)
    }

    #[test]
    fn frac_display() {
        let expected_frac_str = "0.5";
        let num = 1;
        let pow = 2;
        let frac = Frac::new(num, pow);

        let frac_str = format!("{}", frac);
        assert_eq!(frac_str, expected_frac_str)
    }

    #[test]
    fn frac_display_to_from_str() {
        let num = 1;
        let pow = 2;
        let frac = Frac::new(num, pow);

        let frac_str = format!("{}", frac);
        let parsed_frac = frac_str.parse::<Frac>().unwrap();
        assert_eq!(parsed_frac, frac);
    }

    #[test]
    fn test_frac_from_f64() {
        let value = 0.5;
        let frac: Frac = value.into();
        assert_eq!(frac, Frac::new(1, 2)); // 0.5 = 1/2

        let value = 0.125;
        let frac: Frac = value.into();
        assert_eq!(frac, Frac::new(1, 8)); // 0.125 = 1/2^3

        let value = 0.75;
        let frac: Frac = value.into();
        assert_eq!(frac, Frac::new(3, 4)); // 0.75 = 3/2^2

        let value = 3.0;
        let frac: Frac = value.into();
        assert_eq!(frac, Frac::new(3, 1)); // 3.0 = 3/2^0
    }

    #[test]
    fn test_f64_from_frac() {
        let frac = Frac::new(1, 2);
        let value: f64 = frac.into();
        assert_eq!(value, 0.5); // 1/4

        let frac = Frac::new(1, 8);
        let value: f64 = frac.into();
        assert_eq!(value, 0.125); // 1/8

        let frac = Frac::new(3, 8);
        let value: f64 = frac.into();
        assert_eq!(value, 0.375); // 3/8
    }
}
