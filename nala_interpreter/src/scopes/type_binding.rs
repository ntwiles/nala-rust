use crate::{
    ast::types::primitive_type::PrimitiveType,
    errors::RuntimeError,
    resolved::struct_field::StructField,
    types::{type_variant::TypeVariant, NalaType},
};

use super::enum_binding::EnumBinding;

#[derive(Clone, Debug)]
pub enum TypeBinding {
    Enum(EnumBinding, Option<String>),
    Struct(Vec<StructField>, Option<String>),
    Generic(String),
    PrimitiveType(PrimitiveType),
}

impl TypeBinding {
    pub fn from_type(type_variant: TypeVariant, generic_type_param: Option<String>) -> Self {
        match type_variant {
            TypeVariant::Composite(_composite) => todo!(),
            TypeVariant::Type(the_type) => match the_type {
                NalaType::Enum(_ident, variants) => {
                    Self::Enum(EnumBinding { variants }, generic_type_param)
                }
                NalaType::Struct(fields) => Self::Struct(fields, generic_type_param),
                NalaType::Generic(ident) => Self::Generic(ident),
                NalaType::PrimitiveType(primitive) => Self::PrimitiveType(primitive),
            },
        }
    }

    pub fn as_enum(&self) -> Result<(EnumBinding, Option<String>), RuntimeError> {
        match self {
            Self::Enum(binding, type_param) => Ok((binding.clone(), type_param.clone())),
            _ => Err(RuntimeError::new("Expected an enum type.")),
        }
    }

    pub fn as_struct(&self) -> Result<(Vec<StructField>, Option<String>), RuntimeError> {
        match self {
            Self::Struct(fields, type_param) => Ok((fields.clone(), type_param.clone())),
            _ => Err(RuntimeError::new("Expected a struct type.")),
        }
    }

    pub fn get_type_param(&self) -> Result<Option<String>, RuntimeError> {
        match self {
            Self::Enum(_, type_param) => Ok(type_param.clone()),
            Self::Struct(_, type_param) => Ok(type_param.clone()),
            Self::Generic(_) => Ok(None),
            Self::PrimitiveType(_) => Ok(None),
        }
    }
}
