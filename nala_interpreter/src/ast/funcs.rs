use super::{arrays::*, objects::*, terms::*, types::*, *};

#[derive(Debug, Clone)]
pub struct Func {
    pub ident: String,
    pub params: Option<Params>,
    pub block: Box<Block>,
}

// TODO: Implement this as a Vec<Param> instead of a linked list.
// This should remain as a linked list in the grammar.
#[derive(Debug, Clone)]
pub enum Params {
    Params(Box<Params>, Param),
    Param(Param),
    Empty,
}

impl Params {
    pub fn from_vec(params: Vec<Param>) -> Params {
        match params.len() {
            0 => Params::Empty,
            1 => Params::Param(params.first().unwrap().clone()),
            _ => {
                let last = params.last().unwrap();
                let remaining = Params::from_vec(params[..params.len() - 1].to_owned());
                Params::Params(Box::new(remaining), last.clone())
            }
        }
    }

    pub fn to_vec(&self) -> Vec<Param> {
        match self {
            Params::Params(params, param) => {
                let mut params = params.to_vec();
                params.push(param.to_owned());
                params
            }
            Params::Param(param) => vec![param.to_owned()],
            Params::Empty => vec![],
        }
    }
}

#[derive(Debug, Clone)]
pub struct Param {
    pub ident: String,
    pub param_type: TypeVariant,
}

#[derive(Debug, Clone)]
pub enum Call {
    Call(String, Vec<Expr>),
    MemberAccess(MemberAccess),
    Index(Index),
    Term(Term),
}
