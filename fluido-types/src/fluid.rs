use std::{
    fmt::Display,
    num::{ParseFloatError, ParseIntError},
    str::FromStr,
};

use crate::concentration::{Concentration, Volume};

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Fluid {
    concentration: Concentration,
    unit_volume: Volume,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FluidParseError {
    InvalidFloatParse(ParseFloatError),
    InvalidVolumeParse(ParseFloatError),
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

impl FromStr for Fluid {
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

            let concentration = Concentration::from_str(concentration_str)
                .map_err(FluidParseError::InvalidFloatParse)?;
            let unit_volume =
                Volume::from_str(unit_volume_str).map_err(FluidParseError::InvalidVolumeParse)?;

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

impl Display for Fluid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "(")?;
        write!(f, "{}", self.concentration)?;
        write!(f, ",")?;
        write!(f, "{}", self.unit_volume)?;
        write!(f, ")")
    }
}

impl Fluid {
    /// Creates a new fluid.
    ///
    /// Note: Assumes the volume is non-zero.
    pub fn new(concentration: Concentration, unit_volume: Volume) -> Self {
        Self {
            concentration,
            unit_volume,
        }
    }

    /// Mix two fluids, this is a high level representation so it assumes:
    ///  1. Fluids mixes perfectly
    ///  2. Input fluids volumes summed equals to output fluid. (No loss in terms of liquid
    ///     volume).
    pub fn mix(&self, other: &Fluid) -> Self {
        let self_conc: f64 = self.concentration.clone().into();
        let other_conc: f64 = other.concentration.clone().into();

        let self_vol: f64 = self.unit_volume().clone().into();
        let other_vol: f64 = other.unit_volume().clone().into();

        let resulting_vol = self_vol + other_vol;

        let resulting_conc = ((self_conc * self_vol) + (other_conc * other_vol)) / resulting_vol;

        let resulting_conc = Concentration::from(resulting_conc);
        let resulting_vol = Volume::from(resulting_vol);

        Self::new(resulting_conc, resulting_vol)
    }

    /// Returns a reference to the underlying `Concentration` for this fluid.
    pub fn concentration(&self) -> &Concentration {
        &self.concentration
    }

    /// Returns a reference to the underlying unit_volume.
    pub fn unit_volume(&self) -> &Volume {
        &self.unit_volume
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use crate::concentration::{Concentration, Volume};

    use super::Fluid;

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
        let concentration_a = Concentration::from(0.1);
        let voluma_a = Volume::from(1.0);
        let fluid_a = Fluid::new(concentration_a, voluma_a);

        let concentration_b = Concentration::from(0.2);
        let voluma_b = Volume::from(2.0);
        let fluid_b = Fluid::new(concentration_b, voluma_b);

        let resulting_fluid = fluid_a.mix(&fluid_b);
        let expected_concentration = Concentration::from(0.1667);
        let expected_volume = Volume::from(3.0);
        let expected_fluid = Fluid::new(expected_concentration, expected_volume);

        assert_eq!(expected_fluid, resulting_fluid);
    }

    #[test]
    fn parse_fluid_str() {
        let parsed_fluid = Fluid::from_str("(fluid 0.1 1.0)").unwrap();
        let expected_fluid = Fluid::new(0.1.into(), 1.0.into());

        assert_eq!(expected_fluid, parsed_fluid)
    }
}
