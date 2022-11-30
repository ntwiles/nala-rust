use super::{terms::*, types::type_literal_variant::TypeLiteralVariant, *};

#[derive(Debug, Clone)]
pub struct Func {
    pub ident: String,
    pub params: Vec<Param>,
    pub return_type: TypeLiteralVariant,
    pub block: Box<Block>,
}

#[derive(Debug, Clone)]
pub struct Param {
    pub ident: String,
    pub param_type: TypeLiteralVariant,
}

#[derive(Debug, Clone)]
pub enum Invocation {
    Invocation(PlaceExpression, Vec<Expr>),
    PlaceExpression(PlaceExpression),
    Value(Value),
}
