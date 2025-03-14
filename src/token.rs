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

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum TokenKind {
    LAZY,
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
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Token {
    pub kind: TokenKind,
    pub value: String
}

impl Token {
    pub fn new(kind: TokenKind, value: String) -> Token {
        Token {
            kind,
            value,
        }
    }
}
