use crate::{concentration::Concentration, fluid::Fluid};

#[derive(Debug, PartialEq, Clone, Eq, Hash)]
pub enum Expr {
    Mix(Box<Expr>, Box<Expr>),
    LimitedFloat(Concentration),
    Fluid(Fluid),
}
