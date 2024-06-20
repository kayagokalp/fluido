use fluido_types::{fluid::Fluid, number::SaturationNumber};

#[derive(Debug, Clone)]
/// Possible IR operations for mixlang.
pub enum IROp<T: SaturationNumber> {
    // store value_to_store v_register_destination
    Store((Operand<T>, Operand<T>)),
    // mix in1_vreg in2_vreg, target_vreg
    Mix((Operand<T>, Operand<T>, Operand<T>)),
}

#[derive(Debug, Clone)]
pub enum Operand<T: SaturationNumber> {
    Const(Fluid<T>),
    VirtualRegister(usize),
}

impl<T: SaturationNumber> std::fmt::Display for IROp<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            IROp::Store(store) => write!(f, "store {} {}", store.0, store.1),
            IROp::Mix(mix) => write!(f, "mix {} {} {}", mix.0, mix.1, mix.2),
        }
    }
}

impl<T: SaturationNumber> std::fmt::Display for Operand<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Operand::Const(num) => write!(f, "{}", num),
            Operand::VirtualRegister(v_reg) => write!(f, "%{}", v_reg),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::ir::{IROp, Operand};
    use fluido_types::fluid::{Fluid, Frac, LimitedFloat};

    fn get_dummy_fluid_lf() -> Fluid<LimitedFloat> {
        let concentration_a = LimitedFloat::from(0.1);
        let voluma_a = LimitedFloat::from(1.0);
        Fluid::new(concentration_a, voluma_a)
    }
    #[test]
    fn test_operand_display_lf() {
        let const_op = Operand::Const(get_dummy_fluid_lf());
        assert_eq!(format!("{}", const_op), "(fluid 0.1 1.0)");

        let vreg_op: Operand<LimitedFloat> = Operand::VirtualRegister(42);
        assert_eq!(format!("{}", vreg_op), "%42");
    }

    #[test]
    fn test_irops_display_lf() {
        let store_op = IROp::Store((
            Operand::Const(get_dummy_fluid_lf()),
            Operand::VirtualRegister(1),
        ));
        assert_eq!(format!("{}", store_op), "store (fluid 0.1 1.0) %1");

        let mix_op: IROp<LimitedFloat> = IROp::Mix((
            Operand::VirtualRegister(1),
            Operand::VirtualRegister(2),
            Operand::VirtualRegister(3),
        ));
        assert_eq!(format!("{}", mix_op), "mix %1 %2 %3");
    }

    fn get_dummy_fluid_frac() -> Fluid<Frac> {
        let concentration_a = Frac::from(0.1);
        let voluma_a = Frac::from(1.0);
        Fluid::new(concentration_a, voluma_a)
    }
    #[test]
    fn test_operand_display_frac() {
        let const_op = Operand::Const(get_dummy_fluid_frac());
        assert_eq!(format!("{}", const_op), "(fluid 1/10 1)");

        let vreg_op: Operand<Frac> = Operand::VirtualRegister(42);
        assert_eq!(format!("{}", vreg_op), "%42");
    }

    #[test]
    fn test_irops_display_frac() {
        let store_op = IROp::Store((
            Operand::Const(get_dummy_fluid_frac()),
            Operand::VirtualRegister(1),
        ));
        assert_eq!(format!("{}", store_op), "store (fluid 1/10 1) %1");

        let mix_op: IROp<Frac> = IROp::Mix((
            Operand::VirtualRegister(1),
            Operand::VirtualRegister(2),
            Operand::VirtualRegister(3),
        ));
        assert_eq!(format!("{}", mix_op), "mix %1 %2 %3");
    }
}
