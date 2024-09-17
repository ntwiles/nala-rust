use super::{funcs::*, terms::*};

#[derive(Debug, Clone)]
pub enum Addition {
    Add(Box<Addition>, Multiplication),
    Sub(Box<Addition>, Multiplication),
    Multiplication(Multiplication),
}

#[derive(Debug, Clone)]
pub enum Multiplication {
    Mult(Box<Multiplication>, Term),
    Div(Box<Multiplication>, Term),
    Call(Call),
}
