use std::collections::HashMap;

use crate::{
    ast::{
        terms::{FuncValue, Param, Value},
        types::{
            primitive_type::PrimitiveType, type_literal::TypeLiteral,
            type_literal_variant::TypeLiteralVariant,
        },
        FuncVariant,
    },
    errors::RuntimeError,
    io_context::IoContext,
    types::{type_variant::TypeVariant, NalaType},
};

pub fn get_void_block() -> FuncValue {
    let param = Param {
        ident: String::from("_"),
        param_type: TypeVariant::Type(NalaType::PrimitiveType(PrimitiveType::Number)),
    };

    let return_type = TypeLiteralVariant::Type(TypeLiteral::PrimitiveType(PrimitiveType::Void));

    FuncValue {
        params: vec![param],
        return_type,
        type_params: None,
        closure_scope: 0,
        block: Box::new(FuncVariant::Builtin(builtin_void)),
    }
}

fn builtin_void(
    _args: HashMap<String, Value>,
    _ctx: &mut dyn IoContext,
) -> Result<Value, RuntimeError> {
    Ok(Value::Void)
}
