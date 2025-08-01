use std::fmt;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum TokenKind {
    Plus,
    Minus,
    Star,
    Slash,
    LParen,
    RParen,

    Int,

    Error,
    Eof,
}

impl fmt::Display for TokenKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TokenKind::Plus => write!(f, "`+`"),
            TokenKind::Minus => write!(f, "`-`"),
            TokenKind::Star => write!(f, "`*`"),
            TokenKind::Slash => write!(f, "`/`"),
            TokenKind::LParen => write!(f, "`(`"),
            TokenKind::RParen => write!(f, "`)`"),

            TokenKind::Int => write!(f, "integer literal"),

            TokenKind::Error => write!(f, "error"),
            TokenKind::Eof => write!(f, "end of input"),
        }
    }
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum TokenValue {
    None,
    Int(i32),
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Token {
    kind: TokenKind,
    value: TokenValue,
}

impl Token {
    pub fn plus() -> Token {
        Token::simple(TokenKind::Plus)
    }

    pub fn minus() -> Token {
        Token::simple(TokenKind::Minus)
    }

    pub fn star() -> Token {
        Token::simple(TokenKind::Star)
    }

    pub fn slash() -> Token {
        Token::simple(TokenKind::Slash)
    }

    pub fn l_paren() -> Token {
        Token::simple(TokenKind::LParen)
    }

    pub fn r_paren() -> Token {
        Token::simple(TokenKind::RParen)
    }

    pub fn int(value: i32) -> Token {
        Token::new(TokenKind::Int, TokenValue::Int(value))
    }

    pub fn error() -> Token {
        Token::simple(TokenKind::Error)
    }

    pub fn eof() -> Token {
        Token::simple(TokenKind::Eof)
    }

    fn simple(kind: TokenKind) -> Token {
        Token::new(kind, TokenValue::None)
    }

    fn new(kind: TokenKind, value: TokenValue) -> Token {
        Token { kind, value }
    }

    pub fn kind(&self) -> TokenKind {
        self.kind
    }

    pub fn int_value(&self) -> i32 {
        match self.value {
            TokenValue::Int(value) => value,
            _ => panic!("token {self:?} doesn't have an integer value"),
        }
    }
}
