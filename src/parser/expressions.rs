use crate::{ast::expressions::Expr, lexer::token::TokenKind};

use super::{lookup::PRIMARY, parser::{parse_expr, Parser}};

pub fn parse_bracket_expr(p: &mut Parser) -> Expr {
    _ = p.expect(TokenKind::OPEN_BRACKET);
    let mut left = parse_expr(p, PRIMARY)
}