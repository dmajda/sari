use crate::ast::{BinaryOp, Expr};

pub fn int(value: i32) -> Box<Expr> {
    Expr::int(value)
}

pub fn add(left: Box<Expr>, right: Box<Expr>) -> Box<Expr> {
    Expr::binary(BinaryOp::Add, left, right)
}

pub fn sub(left: Box<Expr>, right: Box<Expr>) -> Box<Expr> {
    Expr::binary(BinaryOp::Sub, left, right)
}

pub fn mul(left: Box<Expr>, right: Box<Expr>) -> Box<Expr> {
    Expr::binary(BinaryOp::Mul, left, right)
}

pub fn div(left: Box<Expr>, right: Box<Expr>) -> Box<Expr> {
    Expr::binary(BinaryOp::Div, left, right)
}
