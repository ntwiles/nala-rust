use std::collections::HashMap;
use std::fmt::{Debug, Error, Formatter};

use crate::{
    io_context::IoContext,
    scope::{ScopeId, Scopes},
};

#[derive(Debug)]
pub enum Program {
    Block(Block),
    Stmts(Stmts),
}

#[derive(Clone)]
pub enum Block {
    NalaBlock(Stmts),
    // RustBlock is used for builtin functions.
    RustBlock(Params, BuiltinFunc),
}

impl Debug for Block {
    fn fmt(&self, _formatter: &mut Formatter) -> Result<(), Error> {
        // TODO: Implement this properly.
        Ok(())
    }
}

pub type BuiltinFunc = fn(HashMap<String, Term>, &mut Scopes, ScopeId, &mut dyn IoContext) -> Term;

#[derive(Debug, Clone)]
pub enum Stmt {
    Assign(SymbolOrIndex, Expr),
    Break(Expr),
    Declare(String, Expr, bool),
    Enum(String, Box<KindsDeclare>),
    Expr(Expr),
    For(String, Expr, Box<Block>),
    Func(String, Box<Params>, Box<Block>),
    If(Expr, Box<Block>),
    Wiles(Expr, Box<Block>),
}

#[derive(Debug, Clone)]
pub enum Stmts {
    Stmts(Box<Stmts>, Stmt),
    Stmt(Stmt),
}

#[derive(Debug, Clone)]
pub enum KindsDeclare {
    Kinds(Box<KindsDeclare>, KindDeclare),
    Kind(KindDeclare),
}

#[derive(Debug, Clone)]
pub enum KindDeclare {
    Empty(String),
}

#[derive(Debug, Clone)]
pub struct Array {
    pub elems: Box<Elems>,
}

#[derive(Debug, Clone)]
pub enum Elems {
    Elems(Box<Elems>, Expr),
    Expr(Expr),
    Empty,
}

#[derive(Debug, Clone)]
pub enum Params {
    Params(Box<Params>, Param),
    Param(Param),
    Empty,
}

#[derive(Debug, Clone)]
pub struct Param {
    pub ident: String,
    pub param_type: Type,
}

#[derive(Debug, Clone)]
pub enum Expr {
    Eq(Box<Expr>, KindValue),
    Gt(Box<Expr>, Addend),
    Lt(Box<Expr>, Addend),
    KindValue(KindValue),
    Array(Array),
}

#[derive(Debug, Clone)]
pub enum KindValue {
    KindValue(String, String),
    Addend(Addend),
}

#[derive(Debug, Clone)]
pub enum Addend {
    Add(Box<Addend>, Factor),
    Sub(Box<Addend>, Factor),
    Factor(Factor),
}

#[derive(Debug, Clone)]
pub enum Factor {
    Mult(Box<Factor>, Term),
    Div(Box<Factor>, Term),
    Call(Call),
}

#[derive(Debug, Clone)]
pub enum Call {
    Call(String, Box<Elems>),
    Index(Index),
}

#[derive(Debug, Clone)]
pub enum SymbolOrIndex {
    Symbol(String),
    Index(String, Box<Expr>),
}

#[derive(Debug, Clone)]
pub enum Index {
    Index(String, Box<Expr>),
    Term(Term),
}

#[derive(Debug, Clone)]
pub enum Term {
    Bool(bool),
    Symbol(String),
    String(String),
    Num(f32),
    Array(Vec<Term>),
    Func(Box<Params>, Box<Block>),
    Void,
    Break(Box<Expr>),
    Type(Type),
    Kind(String),
}

#[derive(Debug, Clone)]
pub enum Types {
    Types(Box<Types>, Type),
    Type(Type),
}

#[derive(Debug, Clone)]
pub enum Type {
    Nested(PrimitiveType, Box<Types>),
    Enum(String, Box<KindsDeclare>),
    Primitive(PrimitiveType),
    Interface(PrimitiveInterface),
}

#[derive(Debug, Clone)]
pub enum PrimitiveInterface {
    IPrintable,
}

#[derive(Debug, Clone)]
pub enum PrimitiveType {
    Array,
    Bool,
    Break,
    Func,
    Number,
    String,
    Symbol,
    Void,
    Any,
    Enum,
    Kind,
    Unknown,
}

#[derive(Debug)]
pub enum OpKind {
    Add,
    Sub,
    Mult,
    Div,
}

impl Term {
    pub fn to_string(&self) -> String {
        match self {
            Term::Symbol(_) => {
                panic!("Cannot know string representation of un-evaluated symbol.")
            }
            Term::String(t) => t.to_owned(),
            Term::Num(n) => n.to_string(),
            Term::Bool(b) => b.to_string(),
            Term::Array(a) => String::from(format!("[{}]", a.len())),
            Term::Func(_, _) => String::from(format!("[{}]", self.get_type().to_string())),
            Term::Void => String::from("<Void>"),
            Term::Break(_) => String::from("<Break>"),
            Term::Type(type_kind) => type_kind.to_string(),
            Term::Kind(k) => k.to_owned(),
        }
    }

    pub fn get_type(&self) -> Type {
        match self {
            Term::Array(items) => {
                let elem_type = if items.len() > 0 {
                    items.first().unwrap().get_type()
                } else {
                    Type::Primitive(PrimitiveType::Unknown)
                };

                let elem_type = Types::Type(elem_type);
                Type::Nested(PrimitiveType::Array, Box::new(elem_type))
            }
            Term::Func(params, _) => {
                let params = params.to_vec();
                if params.len() > 0 {
                    let param_types: Vec<Type> =
                        params.iter().map(|p| p.clone().param_type).collect();
                    let param_types = Types::from_vec(param_types);
                    Type::Nested(PrimitiveType::Func, Box::new(param_types))
                } else {
                    Type::Primitive(PrimitiveType::Func)
                }
            }
            Term::Bool(_) => Type::Primitive(PrimitiveType::Bool),
            Term::Break(_) => Type::Primitive(PrimitiveType::Break),
            Term::Num(_) => Type::Primitive(PrimitiveType::Number),
            Term::String(_) => Type::Primitive(PrimitiveType::String),
            Term::Symbol(_) => Type::Primitive(PrimitiveType::Symbol),
            Term::Void => Type::Primitive(PrimitiveType::Void),
            Term::Type(_) => Type::Primitive(PrimitiveType::Enum),
            Term::Kind(_) => Type::Primitive(PrimitiveType::Kind),
        }
    }
}

impl Types {
    pub fn are_assignable_to(&self, other: &Self) -> bool {
        match self {
            Types::Types(svv, sv) => {
                if let Types::Types(ovv, ov) = other {
                    sv.is_assignable_to(ov) && svv.are_assignable_to(ovv)
                } else {
                    false
                }
            }
            Types::Type(sv) => {
                if let Types::Type(ov) = other {
                    sv.is_assignable_to(ov)
                } else {
                    false
                }
            }
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            Types::Type(s) => s.to_string(),
            Types::Types(ss, s) => format!("{0}, {1}", ss.to_string(), s.to_string()),
        }
    }

    pub fn from_vec(types: Vec<Type>) -> Types {
        match types.len() {
            1 => Types::Type(types.first().unwrap().clone()),
            _ => {
                let last = types.last().unwrap();
                let remaining = Types::from_vec(types[..types.len() - 1].to_owned());
                Types::Types(Box::new(remaining), last.clone())
            }
        }
    }
}

impl Type {
    pub fn is_assignable_to(&self, other: &Self) -> bool {
        match self {
            Type::Nested(sv, svv) => {
                if let Type::Nested(ov, ovv) = other {
                    sv.is_assignable_to(ov) && svv.are_assignable_to(ovv)
                } else {
                    false
                }
            }
            Type::Primitive(sv) => {
                if let Type::Primitive(ov) = other {
                    sv.is_assignable_to(ov)
                } else if let Type::Interface(_) = other {
                    // This only works because so far we only have a single interface, IPrintable,
                    // and all primitive types should be treated as IPrintable.
                    true
                } else {
                    false
                }
            }
            Type::Enum(_, _) => todo!(),
            Type::Interface(_) => {
                todo!();
            }
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            Type::Nested(v, vv) => format!("{0}<{1}>", v.to_string(), vv.to_string()),
            Type::Primitive(v) => v.to_string(),
            Type::Enum(_, _) => todo!(),
            Type::Interface(i) => i.to_string(),
        }
    }
}

impl PartialEq for Types {
    fn eq(&self, other: &Self) -> bool {
        return match self {
            Types::Types(mvv, mv) => {
                if let Types::Types(ovv, ov) = other {
                    ovv == mvv && ov == mv
                } else {
                    false
                }
            }
            Types::Type(mv) => {
                if let Types::Type(ov) = other {
                    mv == ov
                } else {
                    false
                }
            }
        };
    }
}

impl PartialEq for Type {
    fn eq(&self, other: &Self) -> bool {
        match self {
            Type::Nested(mv, mg) => {
                if let Type::Nested(ov, og) = other {
                    return mv == ov && mg == og;
                } else {
                    panic!("Cannot compare between generic and primitive types.")
                }
            }
            Type::Primitive(me) => {
                if let Type::Primitive(other) = other {
                    return me == other;
                } else {
                    panic!("Cannot compare between generic and primitive types.");
                }
            }
            Type::Enum(_, _) => {
                todo!()
            }
            Type::Interface(_) => {
                todo!()
            }
        }
    }
}

impl PrimitiveType {
    pub fn is_assignable_to(&self, param: &Self) -> bool {
        param == &PrimitiveType::Any || self.to_string() == param.to_string()
    }

    pub fn to_string(&self) -> String {
        let type_name = match self {
            PrimitiveType::Array => "Array",
            PrimitiveType::Bool => "Bool",
            PrimitiveType::Break => "<Break>",
            PrimitiveType::Func => "Func",
            PrimitiveType::Number => "Number",
            PrimitiveType::String => "String",
            PrimitiveType::Symbol => "<Symbol>",
            PrimitiveType::Void => "<Void>",
            PrimitiveType::Any => "Any",
            PrimitiveType::Enum => "<Enum>",
            PrimitiveType::Kind => "Kind",
            PrimitiveType::Unknown => "<Unknown>",
        };

        String::from(type_name)
    }
}

impl PrimitiveInterface {
    pub fn to_string(&self) -> String {
        let type_name = match self {
            PrimitiveInterface::IPrintable => "IPrintable",
        };

        String::from(type_name)
    }
}

impl PartialEq for PrimitiveType {
    fn eq(&self, other: &Self) -> bool {
        self.to_string() == other.to_string()
    }
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
