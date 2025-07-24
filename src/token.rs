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
