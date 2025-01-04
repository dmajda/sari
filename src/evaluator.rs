use crate::ast::BinaryOp;
use crate::ast::Node;
use crate::error::Error;

pub struct Evaluator<'a> {
    ast: &'a Node,
}

impl Evaluator<'_> {
    pub fn new(ast: &Node) -> Evaluator {
        Evaluator { ast }
    }

    pub fn eval(&self) -> Result<i32, Error> {
        fn eval_node(node: &Node) -> Result<i32, Error> {
            match node {
                Node::IntLit(value) => Ok(*value),
                Node::BinaryExpr { op, left, right } => {
                    let left = eval_node(left)?;
                    let right = eval_node(right)?;

                    match op {
                        BinaryOp::Add => Ok(left.wrapping_add(right)),
                        BinaryOp::Sub => Ok(left.wrapping_sub(right)),
                        BinaryOp::Mul => Ok(left.wrapping_mul(right)),
                        BinaryOp::Div => {
                            if right != 0 {
                                Ok(left.wrapping_div(right))
                            } else {
                                Err(Error::new("division by zero"))
                            }
                        }
                    }
                }
            }
        }

        eval_node(self.ast)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::helpers::ast::*;

    macro_rules! assert_evals {
        ($ast:expr, $value:expr) => {
            let ast = $ast;
            let evaluator = Evaluator::new(&ast);

            assert_eq!(evaluator.eval(), Ok($value));
        };
    }

    macro_rules! assert_does_not_eval {
        ($ast:expr, $message:expr) => {
            let ast = $ast;
            let evaluator = Evaluator::new(&ast);

            assert_eq!(evaluator.eval(), Err(Error::new($message)));
        };
    }

    #[test]
    fn evals_int_lit() {
        assert_evals!(int(1), 1);
    }

    #[test]
    fn evals_binary_expr_add() {
        assert_evals!(add(int(1), int(2)), 3);

        // overflow
        assert_evals!(add(int(2147483647), int(1)), -2147483648);
        assert_evals!(add(int(-2147483648), int(-1)), 2147483647);
    }

    #[test]
    fn evals_binary_expr_sub() {
        assert_evals!(sub(int(3), int(2)), 1);

        // overflow
        assert_evals!(sub(int(2147483647), int(-1)), -2147483648);
        assert_evals!(sub(int(-2147483648), int(1)), 2147483647);
    }

    #[test]
    fn evals_binary_expr_mul() {
        assert_evals!(mul(int(2), int(3)), 6);

        // overflow
        assert_evals!(mul(int(-2147483648), int(-1)), -2147483648);
    }

    #[test]
    fn evals_binary_expr_div() {
        assert_evals!(div(int(6), int(3)), 2);

        // overflow
        assert_evals!(div(int(-2147483648), int(-1)), -2147483648);

        // division by zero
        assert_does_not_eval!(div(int(1), int(0)), "division by zero");
    }

    #[test]
    fn evals_complex_expressions() {
        assert_evals!(mul(add(int(1), int(2)), add(int(3), int(4))), 21);
    }
}
