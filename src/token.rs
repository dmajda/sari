#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Token {
    Plus,
    Minus,
    Star,
    Slash,
    LParen,
    RParen,

    Int(i32),

    Error,
}

impl Token {
    pub fn plus() -> Token {
        Token::Plus
    }

    pub fn minus() -> Token {
        Token::Minus
    }

    pub fn star() -> Token {
        Token::Star
    }

    pub fn slash() -> Token {
        Token::Slash
    }

    pub fn l_paren() -> Token {
        Token::LParen
    }

    pub fn r_paren() -> Token {
        Token::RParen
    }

    pub fn int(value: i32) -> Token {
        Token::Int(value)
    }

    pub fn error() -> Token {
        Token::Error
    }
}
