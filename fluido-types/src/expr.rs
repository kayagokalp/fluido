use crate::fluid::{Fluid, Number};

#[derive(Debug, PartialEq, Clone, Eq, Hash)]
pub enum Expr {
    Mix(Box<Expr>, Box<Expr>),
    Number(Number),
    Fluid(Fluid<Number>),
}
