use crate::source::{Span, Spanned};
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
pub struct IntExpr {
    pub span: Span,
    pub value: i32,
}

impl Spanned for IntExpr {
    fn span(&self) -> Span {
        self.span
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct GroupExpr {
    pub span: Span,
    pub expr: Box<Expr>,
}

impl Spanned for GroupExpr {
    fn span(&self) -> Span {
        self.span
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct BinaryExpr {
    pub span: Span,
    pub op: BinaryOp,
    pub left: Box<Expr>,
    pub right: Box<Expr>,
}

impl Spanned for BinaryExpr {
    fn span(&self) -> Span {
        self.span
    }
}

#[derive(Clone, PartialEq, Debug)]
pub enum Expr {
    Int(IntExpr),
    Group(GroupExpr),
    Binary(BinaryExpr),
}

impl Expr {
    pub fn int(span: Span, value: i32) -> Box<Expr> {
        Box::new(Expr::Int(IntExpr { span, value }))
    }

    pub fn group(span: Span, expr: Box<Expr>) -> Box<Expr> {
        Box::new(Expr::Group(GroupExpr { span, expr }))
    }

    pub fn binary(span: Span, op: BinaryOp, left: Box<Expr>, right: Box<Expr>) -> Box<Expr> {
        Box::new(Expr::Binary(BinaryExpr {
            span,
            op,
            left,
            right,
        }))
    }
}

impl Spanned for Expr {
    fn span(&self) -> Span {
        match self {
            Expr::Int(expr) => expr.span,
            Expr::Group(expr) => expr.span,
            Expr::Binary(expr) => expr.span,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::source::Span;

    #[test]
    fn binary_op_from_token_works() {
        let plus = Token::plus(Span::new(0, 1));
        let minus = Token::minus(Span::new(0, 1));
        let star = Token::star(Span::new(0, 1));
        let slash = Token::slash(Span::new(0, 1));

        assert_eq!(BinaryOp::from_token(plus), BinaryOp::Add);
        assert_eq!(BinaryOp::from_token(minus), BinaryOp::Sub);
        assert_eq!(BinaryOp::from_token(star), BinaryOp::Mul);
        assert_eq!(BinaryOp::from_token(slash), BinaryOp::Div);
    }
}
