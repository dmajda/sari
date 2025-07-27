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
        Token {
            kind: TokenKind::Plus,
            value: TokenValue::None,
        }
    }

    pub fn minus() -> Token {
        Token {
            kind: TokenKind::Minus,
            value: TokenValue::None,
        }
    }

    pub fn star() -> Token {
        Token {
            kind: TokenKind::Star,
            value: TokenValue::None,
        }
    }

    pub fn slash() -> Token {
        Token {
            kind: TokenKind::Slash,
            value: TokenValue::None,
        }
    }

    pub fn l_paren() -> Token {
        Token {
            kind: TokenKind::LParen,
            value: TokenValue::None,
        }
    }

    pub fn r_paren() -> Token {
        Token {
            kind: TokenKind::RParen,
            value: TokenValue::None,
        }
    }

    pub fn int(value: i32) -> Token {
        Token {
            kind: TokenKind::Int,
            value: TokenValue::Int(value),
        }
    }

    pub fn error() -> Token {
        Token {
            kind: TokenKind::Error,
            value: TokenValue::None,
        }
    }

    pub fn eof() -> Token {
        Token {
            kind: TokenKind::Eof,
            value: TokenValue::None,
        }
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
