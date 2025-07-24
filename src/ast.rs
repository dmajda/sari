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
pub enum Node {
    IntLit(i32),
    BinaryExpr {
        op: BinaryOp,
        left: Box<Node>,
        right: Box<Node>,
    },
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
