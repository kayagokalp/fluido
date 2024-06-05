use crate::{concentration::LimitedFloat, fluid::Fluid};

#[derive(Debug, PartialEq, Clone, Eq, Hash)]
pub enum Expr {
    Mix(Box<Expr>, Box<Expr>),
    LimitedFloat(LimitedFloat),
    Fluid(Fluid),
}
