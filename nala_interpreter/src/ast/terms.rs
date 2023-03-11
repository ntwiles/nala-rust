use std::{
    collections::HashMap,
    fmt,
    sync::{Arc, Mutex},
};

use super::*;

#[derive(Debug, Clone)]
pub enum SymbolOrTerm {
    Symbol(String),
    Term(Term),
}

#[derive(Debug, Clone)]
pub enum Term {
    Identifier(String),
    Value(Value),
}

#[derive(Debug, Clone)]
pub struct FuncValue {
    pub params: Vec<Param>,
    pub return_type: TypeLiteralVariant,
    pub block: Box<FuncVariant>,
    pub closure_scope: usize,
}

#[derive(Debug, Clone)]
pub struct EnumVariantValue {
    pub enum_ident: String,
    pub variant_ident: String,
    pub data: Option<Box<Value>>,
}

#[derive(Debug, Clone)]
pub enum Value {
    Array(Arc<Mutex<Vec<Value>>>),
    Bool(bool),
    Func(FuncValue),
    Variant(EnumVariantValue),
    Num(f32),
    Object(Arc<Mutex<HashMap<String, Value>>>),
    String(String),
    Type(TypeLiteralVariant),
    Break(Box<Value>),
    Void,
}

impl Value {
    pub fn is_number(&self) -> bool {
        if let Value::Num(_) = self {
            true
        } else {
            false
        }
    }

    pub fn is_string(&self) -> bool {
        if let Value::String(_) = self {
            true
        } else {
            false
        }
    }

    pub fn is_void(&self) -> bool {
        if let Value::Void = self {
            true
        } else {
            false
        }
    }

    pub fn as_string(&self) -> Option<String> {
        if let Value::String(string) = self {
            Some(string.to_owned())
        } else {
            None
        }
    }

    pub fn as_variant(&self) -> Option<EnumVariantValue> {
        if let Value::Variant(variant) = self {
            Some(variant.clone())
        } else {
            None
        }
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Value::Array(a) => {
                let a = Arc::clone(a);
                let a = a.lock().unwrap();
                write!(f, "<Array[{}]>", a.len())
            }
            Value::Bool(b) => write!(f, "{}", b),
            Value::Break(_) => write!(f, "<Break>"),
            Value::Func(FuncValue {
                params,
                return_type,
                ..
            }) => {
                let mut params: Vec<String> = params
                    .to_vec()
                    .iter()
                    .map(|p| p.param_type.to_string())
                    .collect();

                params.push(return_type.to_string());

                write!(f, "Func<{}>", params.join(", "))
            }
            Value::Num(n) => write!(f, "{}", n),
            Value::Object(fields) => {
                write!(f, "{{ ")?;

                write!(
                    f,
                    "{}",
                    fields
                        .lock()
                        .unwrap()
                        .iter()
                        .map(|(key, value)| format!("{}: {}", key, value))
                        .fold(String::new(), |a, b| a + &b + ", ")
                )?;

                write!(f, "}}")?;

                Ok(())
            }
            // TODO: It's unclear here whether we should wrap this in quotes here.
            // We need them for most cases, but wouldn't want them (for example)
            // when printing out a string using the `print` function.
            Value::String(t) => write!(f, "{}", t),
            Value::Type(type_kind) => write!(f, "{}", type_kind),
            Value::Variant(EnumVariantValue {
                enum_ident,
                variant_ident,
                data,
            }) => {
                let data = if let Some(data) = data {
                    format!("({0})", data)
                } else {
                    "".to_string()
                };

                write!(f, "{0}::{1}{2}", enum_ident, variant_ident, data)
            }
            Value::Void => write!(f, "Void"),
        }
    }
}

impl PartialEq for Value {
    fn eq(&self, right: &Value) -> bool {
        match self {
            Value::Num(left) => {
                if let Value::Num(right) = right {
                    left == right
                } else {
                    false
                }
            }
            Value::String(left) => {
                if let Value::String(right) = right {
                    left == right
                } else {
                    false
                }
            }
            Value::Bool(left) => {
                if let Value::Bool(right) = right {
                    left == right
                } else {
                    false
                }
            }
            _ => todo!(),
        }
    }

    fn ne(&self, _right: &Value) -> bool {
        todo!()
    }
}
