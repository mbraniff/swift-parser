use crate::{lexer::token::{Token, TokenKind}, parser::parser::Parser};

#[derive(Debug)]
pub enum Expr {
    None,
    // Literal Expressions
    FloatExpr(f64),
    IntergerExpr(i64),
    StringExpr(String),
    SymbolExpr(String),

    // Complex Expressions

    /// (Left, Opperator, Right)
    BinaryExpr(Box<Expr>, Token, Box<Expr>),
    /// (Opperator, Right)
    PrefixExpr(Token, Box<Expr>),
    /// (Member, Property)
    MemberExpr(Box<Expr>, String),
    /// (Method, Arguments)
    CallExpr(Box<Expr>, Vec<Box<Expr>>),
    /// (Member, Property)
    ComputedExpr(Box<Expr>, Box<Expr>),
    /// (Lower, Upper)
    RangeExpr(Box<Expr>, Box<Expr>),
    /// (Contents)
    ArrayLiteralExpr(Vec<Box<Expr>>),
}

pub fn parse_primary_expr(p: &mut Parser) -> Expr {
    match p.current_token().kind {
        TokenKind::NUMBER => {
            let token = p.advance();
            if let Some(integer) = token.value.parse::<i64>().ok() {
                return Expr::IntergerExpr(integer);
            }
            if let Some(float) = token.value.parse::<f64>().ok() {
                return Expr::FloatExpr(float);
            }
            panic!("Failed to parse number literal from token {:?}", token);
        },
        TokenKind::STRING => {
            return Expr::StringExpr(p.advance().value.clone());
        },
        TokenKind::IDENTIFIER => {
            return Expr::SymbolExpr(p.advance().value.clone());
        }
        unhandled => {
            panic!("Can not create primary expression from token {:?}", unhandled);
        }
    }
}