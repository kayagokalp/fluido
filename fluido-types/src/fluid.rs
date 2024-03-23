use crate::concentration::Concentration;

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Fluid {
    concentration: Concentration,
    unit_volume: u64,
}

impl Fluid {
    /// Creates a new fluid.
    ///
    /// Note: Assumes the volume is non-zero.
    pub fn new(concentration: Concentration, unit_volume: u64) -> Self {
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

        let resulting_volume = other.unit_volume + self.unit_volume;

        let resulting_conc = ((self_conc * self.unit_volume as f64)
            + (other_conc * other.unit_volume as f64))
            / resulting_volume as f64;

        let resulting_conc = Concentration::from(resulting_conc);

        Self::new(resulting_conc, resulting_volume)
    }
}

#[cfg(test)]
mod tests {
    use crate::concentration::Concentration;

    use super::Fluid;

    #[test]
    fn mix_two_equal_volume_fluids() {
        let concentration_a = Concentration::from(0.1);
        let voluma_a = 1u64;
        let fluid_a = Fluid::new(concentration_a, voluma_a);

        let concentration_b = Concentration::from(0.2);
        let voluma_b = 1u64;
        let fluid_b = Fluid::new(concentration_b, voluma_b);

        let resulting_fluid = fluid_a.mix(&fluid_b);

        let expected_concentration = Concentration::from(0.15);
        let expected_volume = 2u64;
        let expected_fluid = Fluid::new(expected_concentration, expected_volume);

        assert_eq!(expected_fluid, resulting_fluid);
    }

    #[test]
    fn mix_two_diff_volumed_fluids() {
        let concentration_a = Concentration::from(0.1);
        let voluma_a = 1u64;
        let fluid_a = Fluid::new(concentration_a, voluma_a);

        let concentration_b = Concentration::from(0.2);
        let voluma_b = 2u64;
        let fluid_b = Fluid::new(concentration_b, voluma_b);

        let resulting_fluid = fluid_a.mix(&fluid_b);
        let expected_concentration = Concentration::from(0.1667);
        let expected_volume = 3u64;
        let expected_fluid = Fluid::new(expected_concentration, expected_volume);

        assert_eq!(expected_fluid, resulting_fluid);
    }
}
