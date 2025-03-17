use serde::Serialize;

use crate::parser::{parser::Parser, types::nud};

#[derive(Debug, Clone, Serialize)]
pub enum Type {
    Unknown,

    SymbolType {
        modifier: Option<String>,
        value: String,
    },

    ListType {
        underlying: Box<Type>,
    },

    DictType {
        key: Box<Type>,
        value: Box<Type>,
    },

    TupleType {
        values: Vec<Box<Type>>,
    },

    NamedType {
        name: String,
        explicit_type: Box<Type>,
    },

    LabeledType {
        label: String,
        name: String,
        explicit_type: Box<Type>,
    },

    GenericType {
        generics: Vec<Box<Type>>,
        name: String,
    },
}

pub fn parse_type(p: &mut Parser) -> Type {
    let token = p.current_token().clone();
    if let Some(nud_fn) = nud(&token.kind) {
        return (nud_fn)(p);
    }
    panic!("No nud found in type parsing for token {:?}", token.kind);
}