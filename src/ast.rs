use crate::token::Token;

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub enum BinaryOp {
    Add,
    Sub,
    Mul,
    Div,
}

impl BinaryOp {
    pub fn from_token(token: Token) -> BinaryOp {
        match token {
            Token::Add => BinaryOp::Add,
            Token::Sub => BinaryOp::Sub,
            Token::Mul => BinaryOp::Mul,
            Token::Div => BinaryOp::Div,
            _ => panic!("not a binary operator: {:?}", token),
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
        assert_eq!(BinaryOp::from_token(Token::Add), BinaryOp::Add);
        assert_eq!(BinaryOp::from_token(Token::Sub), BinaryOp::Sub);
        assert_eq!(BinaryOp::from_token(Token::Mul), BinaryOp::Mul);
        assert_eq!(BinaryOp::from_token(Token::Div), BinaryOp::Div);
    }
}
