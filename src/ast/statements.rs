use crate::{lexer::token::TokenKind, parser::parser::Parser};

use super::{expressions::Expr, types::Type};

#[derive(Debug)]
pub enum Stmt {
    None,
    /// (Body)
    BlockStmt(Vec<Box<Stmt>>),
    /// (Expression)
    ExpressionStmt(Box<Expr>),
    /// (Identifier, Constant, AssignedValue, ExplicitType)
    VarDeclarationStmt(String, bool, Box<Expr>, Box<Type>),
    /// (ShortName, Name, Type)
    Parameter(String, String, Type),
    /// (Parameters, Name, Body, ReturnType)
    FunctionDeclarationStmt(Vec<Box<Stmt>>, String, Vec<Box<Stmt>>, Type),
    /// (Condition, Consequent, Alternate)
    IfStmt(Box<Expr>, Box<Stmt>, Box<Stmt>),
    /// (Name)
    ImportStmt(String),
    /// (Value, Index, Iterable, Body)
    ForeachStmt(String, bool, Box<Expr>, Vec<Box<Stmt>>),
    /// (Name, Implements, Body)
    ClassDeclarationStmt(String, Vec<String>, Vec<Box<Stmt>>),
}

pub fn parse_var_decl_stmt(p: &mut Parser) -> Stmt {
    let start_token = p.advance().kind;
    let is_constant = start_token == TokenKind::LET;
    let symbol_name = p.advance();

    return Stmt::VarDeclarationStmt(symbol_name.value.clone(), is_constant, Box::new(Expr::None), Box::new(Type::SymbolType(symbol_name.value.clone())));
}