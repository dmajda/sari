use std::iter::Peekable;

use crate::ast::{BinaryOp, Node};
use crate::error::Error;
use crate::scanner::Scanner;
use crate::token::Token;

pub struct Parser<'a> {
    scanner: Peekable<Scanner<'a>>,
}

impl Parser<'_> {
    pub fn new(input: &str) -> Parser {
        Parser {
            scanner: Scanner::new(input).peekable(),
        }
    }

    pub fn parse(&mut self) -> Result<Box<Node>, Error> {
        let expr = self.parse_expr()?;

        match self.scanner.peek() {
            Some(_) => Err(Error::new("expected end of input")),
            None => Ok(expr),
        }
    }

    fn parse_expr(&mut self) -> Result<Box<Node>, Error> {
        let mut term = self.parse_term()?;

        while let Some(&op @ (Token::Plus | Token::Minus)) = self.scanner.peek() {
            self.scanner.next();

            let right = self.parse_term()?;

            term = Box::new(Node::BinaryExpr {
                op: BinaryOp::from_token(op),
                left: term,
                right,
            });
        }

        Ok(term)
    }

    fn parse_term(&mut self) -> Result<Box<Node>, Error> {
        let mut term = self.parse_factor()?;

        while let Some(&op @ (Token::Star | Token::Slash)) = self.scanner.peek() {
            self.scanner.next();

            let right = self.parse_factor()?;

            term = Box::new(Node::BinaryExpr {
                op: BinaryOp::from_token(op),
                left: term,
                right,
            });
        }

        Ok(term)
    }

    fn parse_factor(&mut self) -> Result<Box<Node>, Error> {
        let token = self.scanner.next();

        match token {
            Some(Token::Int(value)) => Ok(Box::new(Node::IntLit(value))),
            Some(Token::LParen) => {
                let expr = self.parse_expr()?;

                match self.scanner.peek() {
                    Some(Token::RParen) => {
                        self.scanner.next();

                        Ok(expr)
                    }
                    _ => Err(Error::new("expected `)`")),
                }
            }
            _ => Err(Error::new("expected integer literal or `(`")),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::helpers::ast::*;

    macro_rules! assert_parses {
        ($input:expr, $ast:expr) => {
            let mut parser = Parser::new($input);

            assert_eq!(parser.parse(), Ok($ast));
        };
    }

    macro_rules! assert_does_not_parse {
        ($input:expr, $error:expr) => {
            let mut parser = Parser::new($input);

            assert_eq!(parser.parse(), Err(Error::new($error)));
        };
    }

    // Canonical expr is `1 + 2`.
    #[test]
    fn parses_expr() {
        assert_parses!("1 * 2", mul(int(1), int(2)));
        assert_parses!(
            "1 * 2 + 3 * 4",
            add(mul(int(1), int(2)), mul(int(3), int(4)))
        );
        assert_parses!(
            "1 * 2 - 3 * 4",
            sub(mul(int(1), int(2)), mul(int(3), int(4)))
        );
        assert_parses!(
            "1 * 2 + 3 * 4 + 5 * 6 + 7 * 8",
            add(
                add(
                    add(mul(int(1), int(2)), mul(int(3), int(4))),
                    mul(int(5), int(6))
                ),
                mul(int(7), int(8))
            )
        );

        // errors
        assert_does_not_parse!("", "expected integer literal or `(`");
        assert_does_not_parse!("1 +", "expected integer literal or `(`");
    }

    // Canonical term is `1 * 2`.
    #[test]
    fn parses_term() {
        assert_parses!("1", int(1));
        assert_parses!("1 * 2", mul(int(1), int(2)));
        assert_parses!("1 / 2", div(int(1), int(2)));
        assert_parses!(
            "1 * 2 * 3 * 4",
            mul(mul(mul(int(1), int(2)), int(3)), int(4))
        );

        // errors
        assert_does_not_parse!("", "expected integer literal or `(`");
        assert_does_not_parse!("1 *", "expected integer literal or `(`");
    }

    // Canonical factor is `1`.
    #[test]
    fn parses_factor() {
        assert_parses!("1", int(1));
        assert_parses!("(1 + 2)", add(int(1), int(2)));

        // errors
        assert_does_not_parse!("(", "expected integer literal or `(`");
        assert_does_not_parse!("(1 + 2", "expected `)`");
    }

    #[test]
    fn parses_complex_expressions() {
        assert_parses!(
            "(1 + 2) * (3 + 4)",
            mul(add(int(1), int(2)), add(int(3), int(4)))
        );
    }

    #[test]
    fn does_not_parse_empty_input() {
        assert_does_not_parse!("", "expected integer literal or `(`");
    }

    #[test]
    fn does_not_parse_trailing_input() {
        assert_does_not_parse!("1 + 2 %", "expected end of input");
    }
}
