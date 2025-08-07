use crate::source::{Span, Spanned};

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

impl TokenKind {
    pub fn as_str(&self) -> &'static str {
        match self {
            TokenKind::Plus => "`+`",
            TokenKind::Minus => "`-`",
            TokenKind::Star => "`*`",
            TokenKind::Slash => "`/`",
            TokenKind::LParen => "`(`",
            TokenKind::RParen => "`)`",

            TokenKind::Int => "integer literal",

            TokenKind::Error => "error",
            TokenKind::Eof => "end of input",
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
    span: Span,
    kind: TokenKind,
    value: TokenValue,
}

impl Token {
    pub fn plus(span: Span) -> Token {
        Token::simple(span, TokenKind::Plus)
    }

    pub fn minus(span: Span) -> Token {
        Token::simple(span, TokenKind::Minus)
    }

    pub fn star(span: Span) -> Token {
        Token::simple(span, TokenKind::Star)
    }

    pub fn slash(span: Span) -> Token {
        Token::simple(span, TokenKind::Slash)
    }

    pub fn l_paren(span: Span) -> Token {
        Token::simple(span, TokenKind::LParen)
    }

    pub fn r_paren(span: Span) -> Token {
        Token::simple(span, TokenKind::RParen)
    }

    pub fn int(span: Span, value: i32) -> Token {
        Token::new(span, TokenKind::Int, TokenValue::Int(value))
    }

    pub fn error(span: Span) -> Token {
        Token::simple(span, TokenKind::Error)
    }

    pub fn eof(span: Span) -> Token {
        Token::simple(span, TokenKind::Eof)
    }

    fn simple(span: Span, kind: TokenKind) -> Token {
        Token::new(span, kind, TokenValue::None)
    }

    fn new(span: Span, kind: TokenKind, value: TokenValue) -> Token {
        Token { span, kind, value }
    }

    pub fn kind(&self) -> TokenKind {
        self.kind
    }

    pub fn int_value(&self) -> i32 {
        let TokenValue::Int(value) = self.value else {
            panic!("token {self:?} doesn't have an integer value")
        };

        value
    }
}

impl Spanned for Token {
    fn span(&self) -> Span {
        self.span
    }
}
