use crate::ast::{PrimitiveInterface::*, *};

pub fn get_interfaces_for_primitive_type(primitive: PrimitiveType) -> Vec<PrimitiveInterface>{
    match primitive {
        PrimitiveType::Array => vec![IPrint],
        PrimitiveType::Bool => vec![IPrint],
        PrimitiveType::Func => vec![IPrint],
        PrimitiveType::Kind => vec![IPrint],
        PrimitiveType::Number => vec![IPrint, ICompare],
        PrimitiveType::String => vec![IPrint],
        _ => vec![]
    }
}