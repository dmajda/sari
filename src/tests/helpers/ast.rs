use crate::ast::{BinaryOp, Node};

pub fn int(value: i32) -> Box<Node> {
    Box::new(Node::IntLit(value))
}

pub fn add(left: Box<Node>, right: Box<Node>) -> Box<Node> {
    Box::new(Node::BinaryExpr {
        op: BinaryOp::Add,
        left,
        right,
    })
}

pub fn sub(left: Box<Node>, right: Box<Node>) -> Box<Node> {
    Box::new(Node::BinaryExpr {
        op: BinaryOp::Sub,
        left,
        right,
    })
}

pub fn mul(left: Box<Node>, right: Box<Node>) -> Box<Node> {
    Box::new(Node::BinaryExpr {
        op: BinaryOp::Mul,
        left,
        right,
    })
}

pub fn div(left: Box<Node>, right: Box<Node>) -> Box<Node> {
    Box::new(Node::BinaryExpr {
        op: BinaryOp::Div,
        left,
        right,
    })
}
