use std::{path::Path, sync::Mutex};

use crate::{ast::{expressions::Expr, statements::Stmt}, lexer::{swift_tokenizer::Tokenizer, token::{token_can_be_name, Token, TokenKind}}, parser::lookup::{bp, led, nud}};

use super::{lookup::{register_lookups, stmt, BindingPower, DEFAULT_BP}, types::register_types_lookup};

static LOOKUPS_MADE: Mutex<bool> = Mutex::new(false);

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

    pub fn go_back(&mut self) {
        self.pos -= 1;
    }

    pub fn has_tokens(&self) -> bool {
        (self.pos as usize) < self.tokens.len() && self.current_token().kind != TokenKind::EOF
    }

    pub fn next_token(&self) -> &Token {
        &self.tokens[(self.pos + 1) as usize]
    }

    pub fn previous_token(&self) -> &Token {
        &self.tokens[(self.pos - 1) as usize]
    }

    pub fn expect(&mut self, kind: TokenKind) -> Token {
        let token = self.advance().clone();
        if token.kind == kind {
            return token;
        }
        panic!("Expected token {:?} but found {:?}", kind, token.kind);
    }

    pub fn has_pattern(&mut self, pattern: &[TokenKind]) -> bool {
        for i in 0..pattern.len() {
            let token = &self.tokens[self.pos as usize + i];
            if pattern[i] == TokenKind::ANYTHING && token_can_be_name(token) {
                continue;
            } else if self.tokens[self.pos as usize + i].kind != pattern[i] {
                return false;
            }
        }
        true
    }
}

pub fn parse(file: &Path, cache: bool) -> Stmt {
    if !*LOOKUPS_MADE.lock().unwrap() {
        *LOOKUPS_MADE.lock().unwrap() = true;
        register_lookups();
        register_types_lookup();
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

    Stmt::BlockStmt{ body }
}

pub fn parse_stmt(p: &mut Parser) -> Stmt {
    if let Some(stmt_fn) = stmt(&p.current_token().kind) {
        let stmt = (stmt_fn)(p);
        if p.current_token().kind == TokenKind::SEMI_COLON { p.advance(); }
        return stmt;
    }

    parse_expr_stmt(p)
}

fn parse_expr_stmt(p: &mut Parser) -> Stmt {
    let expression = parse_expr(p, DEFAULT_BP);
    if p.current_token().kind == TokenKind::SEMI_COLON { p.advance(); }
    Stmt::ExpressionStmt{ expression: Box::new(expression) }
}

pub fn parse_expr(p: &mut Parser, starting_bp: BindingPower) -> Expr {
    let token_kind = p.current_token().kind;

    if let Some(nud) = nud(&token_kind) {
        let mut left = (nud)(p);

        while bp(&token_kind).expect("No bp found") > starting_bp && p.has_tokens() {
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