use std::{collections::HashMap, sync::{Mutex, MutexGuard, OnceLock}};

use crate::{ast::{expressions::{parse_primary_expr, Expr}, statements::{parse_prefix_stmt, parse_var_decl_stmt, Stmt}}, lexer::token::TokenKind};
use super::{expressions::parse_bracket_expr, parser::Parser};

type StmtHandler = fn (p: &mut Parser) -> Stmt;
type NudHandler = fn (p: &mut Parser) -> Expr;
type LedHandler = fn (p: &mut Parser, left: Expr, bp: BindingPower) -> Expr;

pub type BindingPower = u8;
pub const DEFAULT_BP: BindingPower = 0;
pub const COMMA: BindingPower = 1;
pub const ASSIGNMENT: BindingPower = 2;
pub const LOGICAL: BindingPower = 3;
pub const RELATIONAL: BindingPower = 4;
pub const ADDITIVE: BindingPower = 5;
pub const MULTIPLICATIVE: BindingPower = 6;
pub const UNARY: BindingPower = 7;
pub const CALL: BindingPower = 8;
pub const MEMBER: BindingPower = 9;
pub const PRIMARY: BindingPower = 10;

static NUD_LU: OnceLock<Mutex<HashMap<TokenKind, NudHandler>>> = OnceLock::new();
static STMT_LU: OnceLock<Mutex<HashMap<TokenKind, StmtHandler>>> = OnceLock::new();
static BP_LU: OnceLock<Mutex<HashMap<TokenKind, BindingPower>>> = OnceLock::new();
static LED_LU: OnceLock<Mutex<HashMap<TokenKind, LedHandler>>> = OnceLock::new();

pub fn get_map<T>(lu: &'static OnceLock<Mutex<T>>) -> Option<MutexGuard<'static, T>> {
    lu.get()
        .map(|mutex| mutex.lock().unwrap())
}

fn nud_reg(kind: TokenKind, bp: BindingPower, nud_fn: NudHandler) {
    if let Some(bp_map) = &mut get_map(&BP_LU) {
        bp_map.insert(kind, bp);
    }
    if let Some(nud_map) = &mut get_map(&NUD_LU) {
        nud_map.insert(kind, nud_fn);
    }
}

fn stmt_reg(kind: TokenKind, stmt_fn: StmtHandler) {
    if let Some(bp_map) = &mut get_map(&BP_LU) {
        bp_map.insert(kind, DEFAULT_BP);
    }
    if let Some(stmt_map) = &mut get_map(&STMT_LU) {
        stmt_map.insert(kind, stmt_fn);
    }
}

fn led_reg(kind: TokenKind, led_fn: LedHandler) {
    
}

pub fn register_lookups() {
    NUD_LU.set(Mutex::new(HashMap::new())).expect("Registered lookups more than once");
    STMT_LU.set(Mutex::new(HashMap::new())).expect("Registered lookups more than once");
    BP_LU.set(Mutex::new(HashMap::new())).expect("Registered lookups more than once");
    LED_LU.set(Mutex::new(HashMap::new())).expect("Registered lookups more than once");

    nud_reg(TokenKind::NUMBER, PRIMARY, parse_primary_expr);
    nud_reg(TokenKind::STRING, PRIMARY, parse_primary_expr);
    nud_reg(TokenKind::IDENTIFIER, PRIMARY, parse_primary_expr);
    nud_reg(TokenKind::OPEN_BRACKET, PRIMARY, parse_bracket_expr);

    
    stmt_reg(TokenKind::PUBLIC,parse_prefix_stmt);
    stmt_reg(TokenKind::PRIVATE,parse_prefix_stmt);
    stmt_reg(TokenKind::FILEPRIVATE,parse_prefix_stmt);
    stmt_reg(TokenKind::LAZY,parse_prefix_stmt);
    stmt_reg(TokenKind::OPEN,parse_prefix_stmt);
    stmt_reg(TokenKind::INTERNAL,parse_prefix_stmt);
    stmt_reg(TokenKind::STATIC,parse_prefix_stmt);
    stmt_reg(TokenKind::FINAL,parse_prefix_stmt);

    stmt_reg(TokenKind::VAR, parse_var_decl_stmt);
    stmt_reg(TokenKind::LET, parse_var_decl_stmt);
}

pub fn nud(kind: &TokenKind) -> Option<NudHandler> {
    if let Some(nud_map) = get_map(&NUD_LU) {
        return nud_map.get(kind).cloned();
    }
    None
}

pub fn bp(kind: &TokenKind) -> Option<BindingPower> {
    if let Some(bp_map) = get_map(&BP_LU) {
        return bp_map.get(kind).cloned();
    }
    None
}

pub fn led(kind: &TokenKind) -> Option<LedHandler> {
    if let Some(led_map) = get_map(&LED_LU) {
        return led_map.get(kind).cloned();
    }
    None
}

pub fn stmt(kind: &TokenKind) -> Option<StmtHandler> {
    if let Some(stmt_map) = get_map(&STMT_LU) {
        return stmt_map.get(kind).cloned();
    }
    None
}