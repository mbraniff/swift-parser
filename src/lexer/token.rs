use phf::phf_map;
use serde::{Deserialize, Serialize};

static RESERVED_TOKENS: phf::Map<&'static str, TokenKind> = phf_map! {
    "lazy" => TokenKind::LAZY,
    "unowned" => TokenKind::UNOWNED,
    "weak" => TokenKind::WEAK,
    "var" => TokenKind::VAR,
    "let" => TokenKind::LET,
    "import" => TokenKind::IMPORT,
    "open" => TokenKind::OPEN,
    "public" => TokenKind::PUBLIC,
    "internal" => TokenKind::INTERNAL,
    "private" => TokenKind::PRIVATE,
    "fileprivate" => TokenKind::FILEPRIVATE,
    "final" => TokenKind::FINAL,
    "static" => TokenKind::STATIC,
    "class" => TokenKind::CLASS,
    "protocol" => TokenKind::PROTOCOL,
    "actor" => TokenKind::ACTOR,
    "struct" => TokenKind::STRUCT,
    "enum" => TokenKind::ENUM,
    "typealias" => TokenKind::TYPEALIAS,
    "extension" => TokenKind::EXTENSION,
    "func" => TokenKind::FUNC,
    "init" => TokenKind::INIT,
    "deinit" => TokenKind::DEINIT,
    "if" => TokenKind::IF,
    "else" => TokenKind::ELSE,
    "switch" => TokenKind::SWITCH,
    "case" => TokenKind::CASE,
    "default" => TokenKind::DEFAULT,
    "break" => TokenKind::BREAK,
    "continue" => TokenKind::CONTINUE,
    "do" => TokenKind::DO,
    "try" => TokenKind::TRY,
    "catch" => TokenKind::CATCH,
    "throw" => TokenKind::THROW,
    "throws" => TokenKind::THROWS,
    "rethrows" => TokenKind::RETHROWS,
    "guard" => TokenKind::GUARD,
    "repeat" => TokenKind::REPEAT,
    "while" => TokenKind::WHILE,
    "for" => TokenKind::FOR,
    "fallthrough" => TokenKind::FALLTHROUGH,
    "defer" => TokenKind::DEFER,
    "return" => TokenKind::RETURN,
    "in" => TokenKind::IN,
    "where" => TokenKind::WHERE,
    "any" => TokenKind::ANY,
    "some" => TokenKind::SOME,
    "as" => TokenKind::AS,
    "is" => TokenKind::IS,
    "nil" => TokenKind::NIL,
    "true" => TokenKind::TRUE,
    "false" => TokenKind::FALSE,
    "self" => TokenKind::SELF,
    "Self" => TokenKind::TYPE_SELF,
    "Type" => TokenKind::TYPE,
    "super" => TokenKind::SUPER,
};

pub fn string_to_token(symbol: &str) -> &TokenKind {
    let token = RESERVED_TOKENS.get(symbol);
    if let Some(token) = token {
        return token;
    }
    &TokenKind::IDENTIFIER
}

pub fn token_can_be_name(token: &Token) -> bool {
    if RESERVED_TOKENS.contains_key(&token.value) {
        return true;
    }
    return token.kind == TokenKind::IDENTIFIER;
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Copy, Hash)]
#[repr(u64)]
pub enum TokenKind {
    LAZY = 1,
    UNOWNED,
    WEAK,
    VAR,
    LET,
    IDENTIFIER,
    IMPORT,

    // Protections
    OPEN,
    PUBLIC,
    INTERNAL,
    PRIVATE,
    FILEPRIVATE,
    FINAL,
    STATIC,

    // Types
    CLASS,
    PROTOCOL,
    ACTOR,
    STRUCT,
    ENUM,
    TYPEALIAS,
    EXTENSION,

    FUNC,
    INIT,
    DEINIT,

    // CONTROL FLOW
    IF,
    ELSE,
    SWITCH,
    CASE,
    DEFAULT,
    BREAK,
    CONTINUE,
    DO,
    TRY,
    CATCH,
    THROW,
    THROWS,
    RETHROWS,
    GUARD,
    REPEAT,
    WHILE,
    FOR,
    FALLTHROUGH,
    DEFER,
    RETURN,

    IN,
    WHERE,
    ANY,
    SOME,
    AS,
    IS,

    // Existing Types
    NIL,
    TRUE,
    FALSE,
    SELF,
    TYPE_SELF,
    TYPE,
    OPTIONAL,
    DEFAULTING,
    SUPER,

    // Symbols
    OPEN_PAREN,
    CLOSE_PAREN,
    OPEN_BRACKET,
    CLOSE_BRACKET,
    OPEN_BRACE,
    CLOSE_BRACE,
    COLON,
    SEMI_COLON,
    DOT,
    DOT_DOT_DOT,
    RANGE,
    ASSIGNMENT,
    STAR,
    COMMA,
    PLUS_EQUALS,
    MINUS_EQUALS,
    PERCENT,

    // Equalities
    EQUALS,
    GREATER,
    LESS,
    GREATER_EQUALS,
    LESS_EQUALS,
    NOT,
    NOT_EQUALS,
    OR,
    AND,

    ANNOTATION,
    MACRO,
    STRING,
    NUMBER,
    EOF,

    ANYTHING, // A * character to be used to match to alpha reserved types
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Token {
    pub kind: TokenKind,
    pub value: String,
    pub file: String,
    pub line: u64,
    pub col: u64,
}

impl Token {
    pub fn new(kind: TokenKind, value: String, file: String, line: u64, col: u64) -> Token {
        Token {
            kind,
            value,
            file,
            line,
            col
        }
    }
}