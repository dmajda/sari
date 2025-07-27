use crate::token::{Token, TokenKind};

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum BinaryOp {
    Add,
    Sub,
    Mul,
    Div,
}

impl BinaryOp {
    pub fn from_token(token: Token) -> BinaryOp {
        match token.kind() {
            TokenKind::Plus => BinaryOp::Add,
            TokenKind::Minus => BinaryOp::Sub,
            TokenKind::Star => BinaryOp::Mul,
            TokenKind::Slash => BinaryOp::Div,
            _ => panic!("not a binary operator: {token:?}"),
        }
    }
}

#[derive(Clone, PartialEq, Debug)]
pub enum Expr {
    Int(i32),
    Binary {
        op: BinaryOp,
        left: Box<Expr>,
        right: Box<Expr>,
    },
}

impl Expr {
    pub fn int(value: i32) -> Box<Expr> {
        Box::new(Expr::Int(value))
    }

    pub fn binary(op: BinaryOp, left: Box<Expr>, right: Box<Expr>) -> Box<Expr> {
        Box::new(Expr::Binary { op, left, right })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn binary_op_from_token_works() {
        assert_eq!(BinaryOp::from_token(Token::plus()), BinaryOp::Add);
        assert_eq!(BinaryOp::from_token(Token::minus()), BinaryOp::Sub);
        assert_eq!(BinaryOp::from_token(Token::star()), BinaryOp::Mul);
        assert_eq!(BinaryOp::from_token(Token::slash()), BinaryOp::Div);
    }
}
