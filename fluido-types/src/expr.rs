use crate::{fluid::Fluid, number::SaturationNumber};

#[derive(Debug, PartialEq, Clone, Eq, Hash)]
pub enum Expr<T: SaturationNumber> {
    Mix(Box<Expr<T>>, Box<Expr<T>>),
    Number(T),
    Fluid(Fluid<T>),
}
