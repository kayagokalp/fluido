use mixer_generator::concentration::Concentration;

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
    Const(Concentration),
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
