use std::{fmt::Display, str::FromStr};

use crate::number::SaturationNumber;
pub use crate::number::{Frac, LimitedFloat};

pub type Number = Frac;
pub type Concentration = Number;
pub type Volume = Number;

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Fluid<T: SaturationNumber> {
    concentration: T,
    unit_volume: T,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FluidParseError {
    InvalidFloatParse(String),
    InvalidVolumeParse(String),
    MissingParanthesis,
    MissingFluidKeyword,
    MissingSpace,
    MissingVolAndOrConcentration,
}

impl From<FluidParseError> for anyhow::Error {
    fn from(value: FluidParseError) -> Self {
        anyhow::anyhow!(value)
    }
}

impl<T: SaturationNumber> FromStr for Fluid<T> {
    type Err = FluidParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.starts_with('(') && s.ends_with(')') {
            let mut s = s.to_string();
            s.remove(0);
            s.pop();
            let mut split_from_fluid_keyword = s.split("fluid");
            let _ = split_from_fluid_keyword
                .next()
                .ok_or(FluidParseError::MissingFluidKeyword)?;
            let s = split_from_fluid_keyword
                .next()
                .ok_or(FluidParseError::MissingVolAndOrConcentration)?
                .trim();
            let mut splitted_s = s.split(' ');
            let concentration_str = splitted_s
                .next()
                .ok_or(FluidParseError::MissingSpace)?
                .trim();
            let unit_volume_str = splitted_s
                .next()
                .ok_or(FluidParseError::MissingSpace)?
                .trim();

            let concentration = T::parse(concentration_str)
                .map_err(|e| FluidParseError::InvalidFloatParse(e.to_string()))?;
            let unit_volume = T::parse(unit_volume_str)
                .map_err(|e| FluidParseError::InvalidVolumeParse(e.to_string()))?;

            let fluid = Self {
                concentration,
                unit_volume,
            };
            Ok(fluid)
        } else {
            Err(FluidParseError::MissingParanthesis)
        }
    }
}

impl<T: SaturationNumber> Display for Fluid<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "(")?;
        write!(f, "fluid")?;
        write!(f, " ")?;
        write!(f, "{}", self.concentration)?;
        write!(f, " ")?;
        write!(f, "{}", self.unit_volume)?;
        write!(f, ")")
    }
}

// TODO: Make fluid generic over number type.
impl<T: SaturationNumber> Fluid<T> {
    /// Creates a new fluid.
    ///
    /// Note: Assumes the volume is non-zero.
    pub fn new(concentration: T, unit_volume: T) -> Self {
        Self {
            concentration,
            unit_volume,
        }
    }

    /// Mix two fluids, it assumes:
    ///  1. Fluids mixes perfectly
    ///  2. Input fluids volumes summed equals to output fluid. (No loss in terms of liquid
    ///     volume).
    pub fn mix(&self, other: &Fluid<T>) -> Self {
        let self_conc = self.concentration.clone();
        let other_conc = other.concentration.clone();

        let self_vol = self.unit_volume().clone();
        let other_vol = other.unit_volume().clone();

        let resulting_vol = self_vol.clone() + other_vol.clone();

        dbg!(self_conc.clone());
        dbg!(self_vol.clone());
        let self_mult: T = self_conc * self_vol;
        let other_mult: T = other_conc * other_vol;
        let resulting_conc = (self_mult + other_mult) / resulting_vol.clone();

        Self::new(resulting_conc, resulting_vol)
    }

    /// Returns a reference to the underlying `Concentration` for this fluid.
    pub fn concentration(&self) -> &T {
        &self.concentration
    }

    /// Returns a reference to the underlying unit_volume.
    pub fn unit_volume(&self) -> &T {
        &self.unit_volume
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn mix_two_equal_volume_fluids() {
        let concentration_a = Concentration::from(0.1);
        let voluma_a = Volume::from(1.0);
        let fluid_a = Fluid::new(concentration_a, voluma_a);

        let concentration_b = Concentration::from(0.2);
        let voluma_b = Volume::from(1.0);
        let fluid_b = Fluid::new(concentration_b, voluma_b);

        let resulting_fluid = fluid_a.mix(&fluid_b);

        let expected_concentration = Concentration::from(0.15);
        let expected_volume = Volume::from(2.0);
        let expected_fluid = Fluid::new(expected_concentration, expected_volume);

        assert_eq!(expected_fluid, resulting_fluid);
    }

    #[test]
    fn mix_two_diff_volumed_fluids() {
        let concentration_a = Frac::from(0.04);
        let voluma_a = Frac::from(1.0);
        let fluid_a = Fluid::new(concentration_a, voluma_a);

        let concentration_b = Frac::from(0.0);
        let voluma_b = Frac::from(3.0);
        let fluid_b = Fluid::new(concentration_b, voluma_b);

        let resulting_fluid = fluid_a.mix(&fluid_b);
        let expected_concentration = Frac::from(0.01);
        let expected_volume = Frac::from(4.0);
        let expected_fluid = Fluid::new(expected_concentration, expected_volume);

        assert_eq!(expected_fluid, resulting_fluid);
    }

    #[test]
    fn mix_two_equal_volume_fluids_limited_float() {
        let concentration_a = LimitedFloat::from(0.1);
        let voluma_a = LimitedFloat::from(1.0);
        let fluid_a = Fluid::new(concentration_a, voluma_a);

        let concentration_b = LimitedFloat::from(0.2);
        let voluma_b = LimitedFloat::from(1.0);
        let fluid_b = Fluid::new(concentration_b, voluma_b);

        let resulting_fluid = fluid_a.mix(&fluid_b);

        let expected_concentration = LimitedFloat::from(0.15);
        let expected_volume = LimitedFloat::from(2.0);
        let expected_fluid = Fluid::new(expected_concentration, expected_volume);

        assert_eq!(expected_fluid, resulting_fluid);
    }

    #[test]
    fn mix_two_diff_volumed_fluids_limited_float() {
        let concentration_a = LimitedFloat::from(0.04);
        let voluma_a = LimitedFloat::from(1.0);
        let fluid_a = Fluid::new(concentration_a, voluma_a);

        let concentration_b = LimitedFloat::from(0.0);
        let voluma_b = LimitedFloat::from(3.0);
        let fluid_b = Fluid::new(concentration_b, voluma_b);

        let resulting_fluid = fluid_a.mix(&fluid_b);
        let expected_concentration = LimitedFloat::from(0.01);
        let expected_volume = LimitedFloat::from(4.0);
        let expected_fluid = Fluid::new(expected_concentration, expected_volume);

        assert_eq!(expected_fluid, resulting_fluid);
    }

    #[test]
    fn parse_fluid_str() {
        let parsed_fluid: Fluid<LimitedFloat> = Fluid::from_str("(fluid 0.1 1.0)").unwrap();
        let expected_fluid = Fluid::new(0.1.into(), 1.0.into());

        assert_eq!(expected_fluid, parsed_fluid)
    }
}
