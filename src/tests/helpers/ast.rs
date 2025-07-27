use crate::ast::{BinaryOp, Expr};

pub fn int(value: i32) -> Box<Expr> {
    Box::new(Expr::Int(value))
}

pub fn add(left: Box<Expr>, right: Box<Expr>) -> Box<Expr> {
    Box::new(Expr::Binary {
        op: BinaryOp::Add,
        left,
        right,
    })
}

pub fn sub(left: Box<Expr>, right: Box<Expr>) -> Box<Expr> {
    Box::new(Expr::Binary {
        op: BinaryOp::Sub,
        left,
        right,
    })
}

pub fn mul(left: Box<Expr>, right: Box<Expr>) -> Box<Expr> {
    Box::new(Expr::Binary {
        op: BinaryOp::Mul,
        left,
        right,
    })
}

pub fn div(left: Box<Expr>, right: Box<Expr>) -> Box<Expr> {
    Box::new(Expr::Binary {
        op: BinaryOp::Div,
        left,
        right,
    })
}
