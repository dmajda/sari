use crate::ast::{BinaryExpr, BinaryOp, Expr, GroupExpr, IntExpr};
use crate::error::Error;

pub struct Evaluator<'a> {
    ast: &'a Expr,
}

impl Evaluator<'_> {
    pub fn new(ast: &Expr) -> Evaluator {
        Evaluator { ast }
    }

    pub fn eval(&self) -> Result<i32, Error> {
        self.eval_expr(self.ast)
    }

    fn eval_expr(&self, expr: &Expr) -> Result<i32, Error> {
        match expr {
            Expr::Int(expr) => self.eval_int_expr(expr),
            Expr::Group(expr) => self.eval_group_expr(expr),
            Expr::Binary(expr) => self.eval_binary_expr(expr),
        }
    }

    fn eval_int_expr(&self, expr: &IntExpr) -> Result<i32, Error> {
        Ok(expr.value)
    }

    fn eval_group_expr(&self, expr: &GroupExpr) -> Result<i32, Error> {
        self.eval_expr(&expr.expr)
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
                    return Err(Error::new("division by zero"));
                }

                Ok(left.wrapping_div(right))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! assert_evals {
        ($ast:expr, $value:expr $(,)?) => {
            let ast = $ast;
            let evaluator = Evaluator::new(&ast);

            assert_eq!(evaluator.eval(), Ok($value));
        };
    }

    macro_rules! assert_does_not_eval {
        ($ast:expr, $message:expr $(,)?) => {
            let ast = $ast;
            let evaluator = Evaluator::new(&ast);

            assert_eq!(evaluator.eval(), Err(Error::new($message)));
        };
    }

    #[test]
    fn evals_int_expr() {
        assert_evals!(Expr::int(1), 1);
    }

    #[test]
    fn evals_group_expr() {
        assert_evals!(Expr::group(Expr::int(1)), 1);
    }

    #[test]
    fn evals_binary_expr_add() {
        assert_evals!(Expr::binary(BinaryOp::Add, Expr::int(1), Expr::int(2)), 3);

        // overflow
        assert_evals!(
            Expr::binary(BinaryOp::Add, Expr::int(2147483647), Expr::int(1)),
            -2147483648,
        );
        assert_evals!(
            Expr::binary(BinaryOp::Add, Expr::int(-2147483648), Expr::int(-1)),
            2147483647,
        );
    }

    #[test]
    fn evals_binary_expr_sub() {
        assert_evals!(Expr::binary(BinaryOp::Sub, Expr::int(3), Expr::int(2)), 1);

        // overflow
        assert_evals!(
            Expr::binary(BinaryOp::Sub, Expr::int(2147483647), Expr::int(-1)),
            -2147483648,
        );
        assert_evals!(
            Expr::binary(BinaryOp::Sub, Expr::int(-2147483648), Expr::int(1)),
            2147483647,
        );
    }

    #[test]
    fn evals_binary_expr_mul() {
        assert_evals!(Expr::binary(BinaryOp::Mul, Expr::int(2), Expr::int(3)), 6);

        // overflow
        assert_evals!(
            Expr::binary(BinaryOp::Mul, Expr::int(-2147483648), Expr::int(-1)),
            -2147483648,
        );
    }

    #[test]
    fn evals_binary_expr_div() {
        assert_evals!(Expr::binary(BinaryOp::Div, Expr::int(6), Expr::int(3)), 2);

        // overflow
        assert_evals!(
            Expr::binary(BinaryOp::Div, Expr::int(-2147483648), Expr::int(-1)),
            -2147483648,
        );

        // division by zero
        assert_does_not_eval!(
            Expr::binary(BinaryOp::Div, Expr::int(1), Expr::int(0)),
            "division by zero",
        );
    }

    #[test]
    fn evals_complex_expressions() {
        assert_evals!(
            Expr::binary(
                BinaryOp::Mul,
                Expr::group(Expr::binary(BinaryOp::Add, Expr::int(1), Expr::int(2))),
                Expr::group(Expr::binary(BinaryOp::Add, Expr::int(3), Expr::int(4))),
            ),
            21,
        );
    }
}
