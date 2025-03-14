use regex::Regex;
use super::token::{string_to_token, Token, TokenKind};

macro_rules! token_pattern {
    ($pattern:expr, $kind:expr, $value:expr) => {
        RegexPattern::new(
            Regex::new($pattern).unwrap(),
            |lex, _| default_handler(lex, $kind, $value)
        )
    };
}

type RegexHandler = fn (&mut Lexer, &Regex);
#[derive(Clone)]
struct RegexPattern {
    regex: Regex,
    handler: RegexHandler,
}

impl RegexPattern {
    fn new(regex: Regex, handler: RegexHandler) -> RegexPattern {
        RegexPattern { regex, handler }
    }
}

struct Lexer<'a> {
    patterns: Vec<RegexPattern>,
    tokens: Vec<Token>,
    source: &'a str,
    pos: u64,
    line: u64,
}

impl<'a> Lexer<'a> {
    fn new(pos: u64, line: u64, source: &'a str, patterns: Vec<RegexPattern>) -> Lexer<'a> {
        Lexer { patterns, tokens: vec![], source, pos, line }
    }

    fn advance_n(&mut self, n: u64) {
        self.pos += n;
    }

    fn at_eof(&mut self) -> bool {
        self.pos as usize >= self.source.len()
    }

    fn remainder(&mut self) -> &'a str {
        &self.source[self.pos as usize..]
    }

    fn push(&mut self, token: Token) {
        self.tokens.push(token);
    }
}

pub fn tokenize(source: &str) -> Vec<Token> {
    let mut lexer = create_lexer(source);
    let patterns = lexer.patterns.clone();

    while !lexer.at_eof() {
        let mut matched = false;

        for pattern in &patterns {
            if let Some(first_match) = pattern.regex.find(lexer.remainder()) {
                if first_match.start() == 0 {
                    (pattern.handler)(&mut lexer, &pattern.regex);
                    matched = true;
                    break;
                }
            }
        }

        if !matched {
            panic!("lexer error: unrecognized token near {:}", lexer.remainder());
        }
    }

    lexer.push(Token::new(TokenKind::EOF, String::from("")));

    lexer.tokens
}

fn create_lexer(source: &str) -> Lexer {
    let lexer = Lexer::new(0, 1, source, vec![
        RegexPattern::new(Regex::new(r"[^\S\r\n]+").unwrap(), skip_handler),
        RegexPattern::new(Regex::new(r"[\n|\r|\r\n]").unwrap(), new_line_handler),
        RegexPattern::new(Regex::new(r"\/\*[\s\S]*?\*\/").unwrap(), block_comment_handler),
        RegexPattern::new(Regex::new(r"\/\/.*").unwrap(), skip_handler),
        RegexPattern::new(Regex::new(r#""""[\s\S]+?""""#).unwrap(), block_string_handler),
        RegexPattern::new(Regex::new(r#""([^"\\\r\n]*(\\.[^"\\\r\n]*)*)""#).unwrap(), string_handler),
        RegexPattern::new(Regex::new(r"[0-9]+(\.[0-9]+)?").unwrap(), number_handler),
        RegexPattern::new(Regex::new("[a-zA-Z_][a-zA-Z0-9_]*").unwrap(), symbol_handler),
        RegexPattern::new(Regex::new(r"@[a-zA-Z0-9_]*").unwrap(), annotation_handler),
        RegexPattern::new(Regex::new(r"#[a-zA-Z0-9_]*").unwrap(), macro_handler),
        token_pattern!(r"\[", TokenKind::OPEN_BRACKET, "["),
        token_pattern!(r"\]", TokenKind::CLOSE_BRACKET, "]"),
        token_pattern!(r"\{", TokenKind::OPEN_BRACE, "{"),
        token_pattern!(r"\}", TokenKind::CLOSE_BRACE, "}"),
        token_pattern!(r"\(", TokenKind::OPEN_PAREN, "("),
        token_pattern!(r"\)", TokenKind::CLOSE_PAREN, ")"),
        token_pattern!(":", TokenKind::COLON, ":"),
        token_pattern!(";", TokenKind::SEMI_COLON, ";"),
        token_pattern!(r"\.\.\.", TokenKind::DOT_DOT_DOT, "..."),
        token_pattern!(r"\.\.<", TokenKind::RANGE, "..<"),
        token_pattern!(r"\.", TokenKind::DOT, "."),
        token_pattern!(r"\?\?", TokenKind::DEFAULTING, "??"),
        token_pattern!(r"\?", TokenKind::OPTIONAL, "?"),
        token_pattern!(">=", TokenKind::GREATER_EQUALS, ">="),
        token_pattern!("==", TokenKind::EQUALS, "=="),
        token_pattern!(r"\!=", TokenKind::NOT_EQUALS, "!="),
        token_pattern!("<=", TokenKind::LESS_EQUALS, "<="),
        token_pattern!("=", TokenKind::ASSIGNMENT, "="),
        token_pattern!("<", TokenKind::LESS, "<"),
        token_pattern!(">", TokenKind::GREATER, ">"),
        token_pattern!(r"\!", TokenKind::NOT, "!"),
        token_pattern!(r"\*", TokenKind::STAR, "*"),
        token_pattern!(",", TokenKind::COMMA, ","),
        token_pattern!("&&", TokenKind::AND, "&&"),
        token_pattern!(r"\|\|", TokenKind::OR, "||"),
        token_pattern!(r"\+=", TokenKind::PLUS_EQUALS, "+="),
        token_pattern!("-=", TokenKind::MINUS_EQUALS, "-="),
        token_pattern!("%", TokenKind::PERCENT, "%"),
    ]);
    lexer
}

fn default_handler<'a>(lex: &mut Lexer<'a>, kind: TokenKind, value: &'a str) {
    lex.push(Token::new(kind, value.to_string()));
    lex.advance_n(value.len() as u64);
}

fn annotation_handler(lex: &mut Lexer, regex: &Regex) {
    if let Some(first_match) = regex.find(lex.remainder()) {
        lex.push(Token::new(TokenKind::ANNOTATION, first_match.as_str().to_owned()));
        lex.advance_n(first_match.len() as u64);
    }
}

fn macro_handler(lex: &mut Lexer, regex: &Regex) {
    if let Some(first_match) = regex.find(lex.remainder()) {
        lex.push(Token::new(TokenKind::MACRO, first_match.as_str().to_owned()));
        lex.advance_n(first_match.len() as u64);
    }
}

fn symbol_handler(lex: &mut Lexer, regex: &Regex) {
    if let Some(first_match) = regex.find(lex.remainder()) {
        let token = string_to_token(first_match.as_str());
        lex.push(Token::new(token.clone(), first_match.as_str().to_owned()));
        lex.advance_n(first_match.len() as u64);
    }
}

fn number_handler(lex: &mut Lexer, regex: &Regex) {
    if let Some(first_match) = regex.find(lex.remainder()) {
        lex.push(Token::new(TokenKind::NUMBER, first_match.as_str().to_owned()));
        lex.advance_n(first_match.len() as u64);
    }
}

fn new_line_handler(lex: &mut Lexer, regex: &Regex) {
    if let Some(first_match) = regex.find(lex.remainder()) {
        lex.advance_n(first_match.len() as u64);
        lex.line += 1;
    }
}

fn skip_handler(lex: &mut Lexer, regex: &Regex) {
    if let Some(first_match) = regex.find(lex.remainder()) {
        lex.advance_n(first_match.len() as u64);
    }
}

fn block_comment_handler(lex: &mut Lexer, regex: &Regex) {
    if let Some(first_match) = regex.find(lex.remainder()) {
        let lines = first_match.as_str().lines().count() as u64;
        lex.line += lines;
        lex.advance_n(first_match.len() as u64);
    }
}

fn string_handler(lex: &mut Lexer, regex: &Regex) {
    if let Some(first_match) = regex.find(lex.remainder()) {
        lex.push(Token::new(TokenKind::STRING, first_match.as_str().trim_matches('\"').to_owned()));
        lex.advance_n(first_match.len() as u64);
    }
}

fn block_string_handler(lex: &mut Lexer, regex: &Regex) {
    if let Some(first_match) = regex.find(lex.remainder()) {
        lex.push(Token::new(TokenKind::STRING, first_match.as_str()[3..first_match.len()-3].to_owned()));
        lex.advance_n(first_match.len() as u64);
    }
}