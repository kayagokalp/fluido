use fluido_types::fluid::Fluid;

#[derive(Debug, Clone)]
/// Possible IR operations for mixlang.
pub enum IROp {
    // store value_to_store v_register_destination
    Store((Operand, Operand)),
    // mix in1_vreg in2_vreg, target_vreg
    Mix((Operand, Operand, Operand)),
}

#[derive(Debug, Clone)]
pub enum Operand {
    Const(Fluid),
    VirtualRegister(usize),
}

impl std::fmt::Display for IROp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            IROp::Store(store) => write!(f, "store {} {}", store.0, store.1),
            IROp::Mix(mix) => write!(f, "mix {} {} {}", mix.0, mix.1, mix.2),
        }
    }
}

impl std::fmt::Display for Operand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Operand::Const(num) => write!(f, "{}", num),
            Operand::VirtualRegister(v_reg) => write!(f, "%{}", v_reg),
        }
    }
}

#[cfg(test)]
mod tests {

    #[cfg(test)]
    mod tests {
        use crate::ir::{IROp, Operand};
        use fluido_types::fluid::{Concentration, Fluid, Volume};

        fn get_dummy_fluid() -> Fluid {
            let concentration_a = Concentration::from(0.1);
            let voluma_a = Volume::from(1.0);
            Fluid::new(concentration_a, voluma_a)
        }
        #[test]
        fn test_operand_display() {
            let const_op = Operand::Const(get_dummy_fluid());
            assert_eq!(format!("{}", const_op), "(fluid 0.1 1.0)");

            let vreg_op = Operand::VirtualRegister(42);
            assert_eq!(format!("{}", vreg_op), "%42");
        }

        #[test]
        fn test_irops_display() {
            let store_op = IROp::Store((
                Operand::Const(get_dummy_fluid()),
                Operand::VirtualRegister(1),
            ));
            assert_eq!(format!("{}", store_op), "store (fluid 0.1 1.0) %1");

            let mix_op = IROp::Mix((
                Operand::VirtualRegister(1),
                Operand::VirtualRegister(2),
                Operand::VirtualRegister(3),
            ));
            assert_eq!(format!("{}", mix_op), "mix %1 %2 %3");
        }
    }
}
