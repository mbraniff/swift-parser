use serde::Serialize;

use crate::{lexer::token::{Token, TokenKind}, parser::{lookup::UNARY, parser::{parse_expr, Parser}}};

#[derive(Debug, Serialize)]
pub enum Expr {
    None,
    // Literal Expressions
    FloatExpr {
        value: f64,
    },
    IntergerExpr {
        value: i64,
    },
    StringExpr {
        value: String,
    },
    SymbolExpr {
        value: String,
    },

    // Complex Expressions
    BinaryExpr {
        left: Box<Expr>,
        opperator: Token,
        right: Box<Expr>,
    },

    PrefixExpr {
        opperator: Token,
        right: Box<Expr>,
    },

    MemberExpr {
        member: Box<Expr>,
        property: String,
    },

    CallExpr {
        method: Box<Expr>,
        arguments: Vec<Box<Expr>>,
    },

    ComputedExpr {
        member: Box<Expr>,
        property: Box<Expr>,
    },

    RangeExpr {
        lower: Box<Expr>,
        upper: Box<Expr>,
    },

    ArrayLiteralExpr {
        contents: Vec<Box<Expr>>,
    },
}

pub fn parse_primary_expr(p: &mut Parser) -> Expr {
    match p.current_token().kind {
        TokenKind::NUMBER => {
            let token = p.advance();
            if let Some(integer) = token.value.parse::<i64>().ok() {
                return Expr::IntergerExpr { value: integer };
            }
            if let Some(float) = token.value.parse::<f64>().ok() {
                return Expr::FloatExpr { value: float };
            }
            panic!("Failed to parse number literal from token {:?}", token);
        },
        TokenKind::STRING => {
            return Expr::StringExpr { value: p.advance().value.clone() };
        },
        TokenKind::IDENTIFIER => {
            return Expr::SymbolExpr { value: p.advance().value.clone() };
        }
        unhandled => {
            panic!("Can not create primary expression from token {:?}", unhandled);
        }
    }
}

pub fn parse_prefix_expr(p: &mut Parser) -> Expr {
    let opperator = p.advance().clone();
    let expr = parse_expr(p, UNARY);
    Expr::PrefixExpr { opperator, right: Box::new(expr) }
}