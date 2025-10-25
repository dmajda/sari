use std::cell::RefCell;
use std::rc::Rc;

use crate::ast::{BinaryExpr, BinaryOp, Expr, GroupExpr, IntExpr, UnaryExpr, UnaryOp};
use crate::error::Error;
use crate::source::{SourceMap, SourceSpan, Span, Spanned};

pub struct Evaluator<'a> {
    ast: &'a Expr,
    source_map: Rc<RefCell<SourceMap>>,
}

impl Evaluator<'_> {
    pub fn new(ast: &Expr, source_map: Rc<RefCell<SourceMap>>) -> Evaluator<'_> {
        Evaluator { ast, source_map }
    }

    pub fn eval(&self) -> Result<i32, Error> {
        self.eval_expr(self.ast)
    }

    fn eval_expr(&self, expr: &Expr) -> Result<i32, Error> {
        match expr {
            Expr::Int(expr) => self.eval_int_expr(expr),
            Expr::Group(expr) => self.eval_group_expr(expr),
            Expr::Unary(expr) => self.eval_unary_expr(expr),
            Expr::Binary(expr) => self.eval_binary_expr(expr),
        }
    }

    fn eval_int_expr(&self, expr: &IntExpr) -> Result<i32, Error> {
        Ok(expr.value)
    }

    fn eval_group_expr(&self, expr: &GroupExpr) -> Result<i32, Error> {
        self.eval_expr(&expr.expr)
    }

    fn eval_unary_expr(&self, expr: &UnaryExpr) -> Result<i32, Error> {
        let expr_inner = self.eval_expr(&expr.expr)?;

        match expr.op {
            UnaryOp::Pos => Ok(expr_inner),
            UnaryOp::Neg => Ok(expr_inner.wrapping_neg()),
        }
    }

    fn eval_binary_expr(&self, expr: &BinaryExpr) -> Result<i32, Error> {
        let left = self.eval_expr(&expr.left)?;
        let right = self.eval_expr(&expr.right)?;

        match expr.op {
            BinaryOp::Add => Ok(left.wrapping_add(right)),
            BinaryOp::Sub => Ok(left.wrapping_sub(right)),
            BinaryOp::Mul => Ok(left.wrapping_mul(right)),
            BinaryOp::Div => {
                if right == 0 {
                    return Err(self.error(expr, "division by zero"));
                }

                Ok(left.wrapping_div(right))
            }
        }
    }

    fn error(&self, spanned: &impl Spanned, message: impl Into<String>) -> Error {
        Error::new(self.map_span(spanned.span()), message)
    }

    fn map_span(&self, span: Span) -> SourceSpan {
        self.source_map.borrow().map_span(span)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::Parser;
    use crate::source::SourcePos;

    macro_rules! assert_evals {
        ($input:expr, $value:expr $(,)?) => {
            let source_map = Rc::new(RefCell::new(SourceMap::new()));

            let ast = Parser::new($input, Rc::clone(&source_map)).parse().unwrap();
            let evaluator = Evaluator::new(&ast, Rc::clone(&source_map));

            assert_eq!(evaluator.eval(), Ok($value));
        };
    }

    macro_rules! assert_does_not_eval {
        ($input:expr, $error:expr $(,)?) => {
            let source_map = Rc::new(RefCell::new(SourceMap::new()));

            let ast = Parser::new($input, Rc::clone(&source_map)).parse().unwrap();
            let evaluator = Evaluator::new(&ast, Rc::clone(&source_map));

            assert_eq!(evaluator.eval(), Err($error));
        };
    }

    #[test]
    fn evals_int_expr() {
        assert_evals!("1", 1);
    }

    #[test]
    fn evals_group_expr() {
        assert_evals!("(1)", 1);
    }

    #[test]
    fn evals_unary_expr_pos() {
        assert_evals!("+1", 1);
    }

    #[test]
    fn evals_unary_expr_neg() {
        assert_evals!("-1", -1);

        // overflow
        assert_evals!("-2147483648", -2147483648);
    }

    #[test]
    fn evals_binary_expr_add() {
        assert_evals!("1 + 2", 3);

        // overflow
        assert_evals!("2147483647 + 1", -2147483648);
        assert_evals!("-2147483648 + -1", 2147483647);
    }

    #[test]
    fn evals_binary_expr_sub() {
        assert_evals!("3 - 2", 1);

        // overflow
        assert_evals!("2147483647 - -1", -2147483648);
        assert_evals!("-2147483648 - 1", 2147483647);
    }

    #[test]
    fn evals_binary_expr_mul() {
        assert_evals!("2 * 3", 6);

        // overflow
        assert_evals!("-2147483648 * -1", -2147483648);
    }

    #[test]
    fn evals_binary_expr_div() {
        assert_evals!("6 / 3", 2);

        // overflow
        assert_evals!("-2147483648 / -1", -2147483648);

        // division by zero
        assert_does_not_eval!(
            "1 / 0",
            Error::new(
                SourceSpan::new(SourcePos::new(0, 1, 1), SourcePos::new(5, 1, 6)),
                "division by zero",
            ),
        );
    }

    #[test]
    fn evals_complex_expressions() {
        assert_evals!("(1 + 2) * (3 + 4)", 21);
    }
}
