use serde::Serialize;

use crate::{lexer::token::TokenKind, parser::{lookup::ASSIGNMENT, parser::{parse_expr, parse_stmt, Parser}}};

use super::{expressions::Expr, types::{parse_type, Type}};

#[derive(Debug, Serialize)]
pub enum Stmt {
    None,

    BlockStmt {
        body: Vec<Box<Stmt>>,
    },

    ExpressionStmt {
        expression: Box<Expr>,
    },

    VarDeclarationStmt {
        modifiers: Vec<String>,
        identifier: String,
        constant: bool,
        assigned_value: Box<Expr>,
        explicit_type: Box<Type>,
    },

    Parameter {
        explicit_type: Box<Type>,
    },

    FunctionDeclarationStmt {
        parameters: Vec<Box<Type>>,
        name: String,
        body: Vec<Box<Stmt>>,
        return_type: Box<Type>,
    },

    IfStmt {
        condition: Box<Expr>,
        consequent: Box<Stmt>,
        alternate: Box<Stmt>,
    },

    ImportStmt {
        name: String,
    },

    ForeachStmt {
        value: String,
        index: bool,
        iterable: Box<Expr>,
        body: Box<Stmt>,
    },

    ClassDeclarationStmt {
        name: String,
        implements: Vec<String>,
        body: Box<Stmt>,
    },
}

fn is_modifier(token: &TokenKind) -> bool {
    match token {
        TokenKind::FILEPRIVATE |
        TokenKind::PRIVATE |
        TokenKind::INTERNAL |
        TokenKind::PUBLIC|
        TokenKind::OPEN |
        TokenKind::STATIC |
        TokenKind:: LAZY => return true,
        _ => return false
    }
}

pub fn parse_var_decl_stmt(p: &mut Parser) -> Stmt {
    let start_token = p.advance().kind;
    let is_constant = start_token == TokenKind::LET;
    let symbol_name = p.advance().value.clone();

    let mut token = p.current_token();
    let mut explicit_type = Type::Unknown;
    match token.kind {
        TokenKind::COLON => {
            p.advance();
            explicit_type = parse_type(p);
        }
        _ => {}
    }
    token = p.current_token();
    let mut assignment = Expr::None;
    match token.kind {
        TokenKind::ASSIGNMENT => {
            p.advance();
            assignment = parse_expr(p, ASSIGNMENT);
        }
        _ => {}
    }
    match explicit_type {
        Type::Unknown => {
            match assignment {
                Expr::FloatExpr{..} => { explicit_type = Type::SymbolType { modifier: None, value: String::from("Double") } },
                Expr::IntergerExpr{..} => { explicit_type = Type::SymbolType { modifier: None, value: String::from("Int") } },
                Expr::StringExpr{..} => { explicit_type = Type::SymbolType { modifier: None, value: String::from("String") } },
                Expr::SymbolExpr{ref value} => { explicit_type = Type::SymbolType { modifier: None, value: value.to_string() } },
                _ => {}
            }
        }
        _ => {}
    }

    return Stmt::VarDeclarationStmt { modifiers: vec![], identifier: symbol_name.clone(), constant: is_constant, assigned_value: Box::new(assignment), explicit_type: Box::new(explicit_type) };
}

pub fn parse_prefix_stmt(p: &mut Parser) -> Stmt {
    let mut modifiers = vec![];
    while p.has_tokens() && is_modifier(&p.current_token().kind) {
        modifiers.push(p.advance().value.clone());
    }
    let mut stmt = parse_stmt(p);
    match &mut stmt {
        Stmt::VarDeclarationStmt { modifiers: existing_modifiers, ..} => {
            existing_modifiers.append(&mut modifiers);
        }
        _ => {}
    }
    stmt
}