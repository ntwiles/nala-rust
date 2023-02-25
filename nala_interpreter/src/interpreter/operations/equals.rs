use crate::{
    ast::{terms::*, types::primitive_type::PrimitiveType},
    scope::{ScopeId, Scopes},
    types::{type_variant::TypeVariant, NalaType},
};

use super::errors::panic_oper_not_impl_for;

pub fn eval_equals(
    left: Value,
    right: Value,
    scopes: &mut Scopes,
    current_scope: ScopeId,
) -> Value {
    if left.get_type(scopes, current_scope) != right.get_type(scopes, current_scope) {
        panic_oper_not_impl_for(
            "==",
            &TypeVariant::Type(NalaType::PrimitiveType(PrimitiveType::Number)),
            &right.get_type(scopes, current_scope),
        )
    }

    match left {
        Value::Variant(left_enum, left_variant, data) => {
            variant_equals(left_enum, left_variant, data, right, scopes, current_scope)
        }
        _ => Value::Bool(left == right),
    }
}

fn variant_equals(
    left_enum: String,
    left_variant: String,
    left_data: Option<Box<Value>>,
    right: Value,
    _scopes: &mut Scopes,
    _current_scope: ScopeId,
) -> Value {
    if let Value::Variant(right_enum, right_variant, right_data) = right {
        let enums_match = left_enum == right_enum;
        let variants_match = left_variant == right_variant;

        if let Some(right_data) = right_data {
            let data_matches = if let Some(left_data) = left_data {
                left_data == right_data
            } else {
                false
            };

            Value::Bool(enums_match && variants_match && data_matches)
        } else {
            Value::Bool(enums_match && variants_match)
        }
    } else {
        todo!()
        // panic_oper_not_impl_for(
        //     "==",
        //     &TypeVariant::Type(NalaType::UserDefined(left_enum)),
        //     &right.get_type(),
        // )
    }
}
