use std::{path::Path, sync::Mutex};

use crate::{ast::{expressions::Expr, statements::Stmt}, lexer::{swift_tokenizer::Tokenizer, token::{Token, TokenKind}}, parser::lookup::{bp, led, nud}};

use super::lookup::{register_lookups, stmt, BindingPower, DEFAULT_BP};

static lookups_made: Mutex<bool> = Mutex::new(false);

pub struct Parser {
    tokens: Vec<Token>,
    pos: u64,
}

impl Parser {
    fn new(tokens: Vec<Token>) -> Parser {
        Parser {
            tokens,
            pos: 0
        }
    }

    pub fn current_token(&self) -> &Token {
        &self.tokens[self.pos as usize]
    }

    pub fn advance(&mut self) -> &Token {
        let token = &self.tokens[self.pos as usize];
        self.pos += 1;
        return token;
    }

    fn has_tokens(&self) -> bool {
        (self.pos as usize) < self.tokens.len() && self.current_token().kind != TokenKind::EOF
    }

    pub fn next_token(&self) -> &Token {
        &self.tokens[(self.pos + 1) as usize]
    }

    pub fn previous_token(&self) -> &Token {
        &self.tokens[(self.pos - 1) as usize]
    }
}

pub fn parse(file: &Path, cache: bool) -> Stmt {
    if !*lookups_made.lock().unwrap() {
        *lookups_made.lock().unwrap() = true;
        register_lookups();
    }

    let mut tokenizer: Tokenizer;
    if cache {
        tokenizer = Tokenizer::new_cached(Path::new("./cache.txt"));
    } else {
        tokenizer = Tokenizer::new_non_cached();
    }
    let tokens = tokenizer.tokenize(file).expect("Failed to tokenize file");
    let mut parser = Parser::new(tokens);
    let mut body = vec![];

    while parser.has_tokens() {
        body.push(Box::new(parse_stmt(&mut parser)));
    }

    Stmt::BlockStmt(body)
}

fn parse_stmt(p: &mut Parser) -> Stmt {
    if let Some(stmt_fn) = stmt(&p.current_token().kind) {
        return (stmt_fn)(p);
    }

    parse_expr_stmt(p)
}

fn parse_expr_stmt(p: &mut Parser) -> Stmt {
    let expression = parse_expr(p, DEFAULT_BP);
    Stmt::ExpressionStmt(Box::new(expression))
}

fn parse_expr(p: &mut Parser, starting_bp: BindingPower) -> Expr {
    let token_kind = p.current_token().kind;

    if let Some(nud) = nud(&token_kind) {
        let mut left = (nud)(p);

        while bp(&token_kind).expect("No bp found") > starting_bp {
            let token_kind = p.current_token().kind;
            if let Some(led_fn) = led(&token_kind) {
                left = led_fn(p, left, starting_bp);
            } else {
                panic!("Led handler expected for token {:?}", token_kind);
            }
        }

        return left
    }

    panic!("Nud handler expected for token {:?}", token_kind);
}