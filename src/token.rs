#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Token {
    Add,
    Sub,
    Mul,
    Div,
    LParen,
    RParen,

    Int(i32),

    Error(char),
}
