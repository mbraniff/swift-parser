use std::{collections::HashMap, sync::{Mutex, OnceLock}};

use crate::{ast::types::{parse_type, Type}, lexer::token::TokenKind};

use super::{lookup::{get_map, BindingPower, MEMBER, PRIMARY}, parser::Parser};

type NudHandler = fn (p: &mut Parser) -> Type;
type LedHandler = fn (p: &mut Parser, left: Type, bp: BindingPower) -> Type;

static NUD_LU: OnceLock<Mutex<HashMap<TokenKind, NudHandler>>> = OnceLock::new();
static LED_LU: OnceLock<Mutex<HashMap<TokenKind, LedHandler>>> = OnceLock::new();
static BP_LU: OnceLock<Mutex<HashMap<TokenKind, BindingPower>>> = OnceLock::new();

fn led_reg(kind: TokenKind, bp: BindingPower, led_fn: LedHandler) {
    if let Some(bp_map) = &mut get_map(&BP_LU) {
        bp_map.insert(kind, bp);
    }
    if let Some(led_map) = &mut get_map(&LED_LU) {
        led_map.insert(kind, led_fn);
    }
}

fn nud_reg(kind: TokenKind, bp: BindingPower, nud_fn: NudHandler) {
    if let Some(bp_map) = &mut get_map(&BP_LU) {
        bp_map.insert(kind, bp);
    }
    if let Some(nud_map) = &mut get_map(&NUD_LU) {
        nud_map.insert(kind, nud_fn);
    }
}

pub fn register_types_lookup() {
    NUD_LU.set(Mutex::new(HashMap::new())).expect("Registered lookups more than once");
    LED_LU.set(Mutex::new(HashMap::new())).expect("Registered lookups more than once");
    BP_LU.set(Mutex::new(HashMap::new())).expect("Registered lookups more than once");

    nud_reg(TokenKind::IDENTIFIER, PRIMARY, parse_identifier_type);

    nud_reg(TokenKind::OPEN_BRACKET, MEMBER, parse_bracket_type);
    nud_reg(TokenKind::OPEN_PAREN, MEMBER, parse_tuple_type);
    nud_reg(TokenKind::ANY, PRIMARY, parse_prefixed_type);
    nud_reg(TokenKind::SOME, PRIMARY, parse_prefixed_type);
}

pub fn nud(kind: &TokenKind) -> Option<NudHandler> {
    if let Some(nud_map) = get_map(&NUD_LU) {
        return nud_map.get(kind).cloned();
    }
    None
}

fn parse_identifier_type(p: &mut Parser) -> Type {
    let token = p.advance().clone();
    if p.current_token().kind == TokenKind::LESS {
        let generics = parse_generic_types(p);
        return Type::GenericType { generics, name: token.value };
    }
    Type::SymbolType { modifier: None, value: token.value }
}

fn parse_generic_types(p: &mut Parser) -> Vec<Box<Type>> {
    let mut result = vec![];
    _ = p.expect(TokenKind::LESS);
    result.push(Box::new(parse_type(p)));
    while p.current_token().kind == TokenKind::COMMA {
        _ = p.advance();
        result.push(Box::new(parse_type(p)));
    }
    p.expect(TokenKind::GREATER);
    result
}

fn parse_prefixed_type(p: &mut Parser) -> Type {
    let prefix = p.advance().clone();
    match prefix.kind {
        TokenKind::ANY |
        TokenKind::SOME => {},
        _ => { panic!("Unexpected prefix token {:?} found", prefix.kind); }
    }
    let explicit_type = parse_type(p);
    if let Type::SymbolType { modifier: existing_prefix, value: explicit_type } = explicit_type {
        if let Some(existing_prefix) = existing_prefix {
            panic!("Prefixed types can only have a single prefix. Found {:?} and {:?} before {:?}", prefix.value, existing_prefix, explicit_type);
        }
        return Type::SymbolType { modifier: Some(prefix.value.clone()), value: explicit_type };
    } 
    panic!("Expect a symbol type following a prefix, found {:?}", explicit_type);
}

fn parse_bracket_type(p: &mut Parser) -> Type {
    let _ = p.expect(TokenKind::OPEN_BRACKET);
    let first_type = parse_type(p);
    let token = p.advance();
    match token.kind {
        TokenKind::COLON => {
            let second_type = parse_type(p);
            _ = p.expect(TokenKind::CLOSE_BRACKET);
            return Type::DictType { key: Box::new(first_type), value: Box::new(second_type) };
        },
        TokenKind::CLOSE_BRACKET => {
            return Type::ListType { underlying: Box::new(first_type) };
        },
        _ => { panic!("Unexpected token, {:?}, found while parsing bracket type", token.kind); }
    }
}

fn parse_tuple_type(p: &mut Parser) -> Type {
    let _ = p.expect(TokenKind::OPEN_PAREN);
    let first_type = parse_named_type(p, "0");
    let mut token = p.advance().clone();
    if token.kind == TokenKind::CLOSE_PAREN {
        match first_type {
            Type::NamedType { name, explicit_type } => {
                if name != "0" {
                    panic!("Can not create single element tuple with element label");
                }
                return *explicit_type;
            },
            _ => { panic!("Named type expected in tuple but found {:?} instead", first_type); }
        }
    }

    let mut named_types = vec![Box::new(first_type)];
    let mut count = 1;
    while token.kind == TokenKind::COMMA {
        named_types.push(Box::new(parse_named_type(p, format!("{count}").as_str())));
        count += 1;
        token = p.advance().clone();
    }

    if token.kind == TokenKind::CLOSE_PAREN {
        return Type::TupleType { values: named_types };
    }
    panic!("Unexpected token, {:?}, found while completing the parsing of a tuple type", token);
}

pub fn parse_named_type(p: &mut Parser, default_name: &str) -> Type {
    if p.has_pattern(&[TokenKind::ANYTHING, TokenKind::COLON]) { // Named
        let name = p.advance().clone();
        _ = p.expect(TokenKind::COLON);
        let explicit_type = parse_type(p);
        return Type::NamedType { name: name.value.clone(), explicit_type: Box::new(explicit_type) };
    }
    if p.has_pattern(&[TokenKind::IDENTIFIER, TokenKind::IDENTIFIER]) { // Labeled 
        panic!("Parsing named type but found 2 labels");    
    }
    let ref first = parse_type(p);
    match first {
        Type::SymbolType{..} => {
            let next = p.advance().clone();
            match next.kind {
                TokenKind::COMMA |
                TokenKind::CLOSE_PAREN => {
                    p.go_back();
                    return Type::NamedType { name: default_name.to_string(), explicit_type: Box::new(first.clone()) };
                },
                _ => { panic!("Unexpected token, {:?}, found while parsing named type", next.kind); }
            }
        },
        Type::DictType{..} |
        Type::TupleType{..} |
        Type::ListType{..} => {
            return Type::NamedType { name: default_name.to_string(), explicit_type: Box::new(first.clone()) };
        }
        _ => { panic!("Unexpected type, {:?}, found while beginning to parse named type", first) }
    }
}