use std::mem;

use crate::ast::{BinaryOp, Expr};
use crate::error::Error;
use crate::scanner::Scanner;
use crate::token::{Token, TokenKind};

pub struct Parser<'a> {
    scanner: Scanner<'a>,
    current: Token,
}

impl Parser<'_> {
    pub fn new(input: &str) -> Parser {
        Parser {
            scanner: Scanner::new(input),
            current: Token::eof(),
        }
    }

    pub fn parse(&mut self) -> Result<Box<Expr>, Error> {
        self.advance();

        let expr = self.parse_expr()?;
        self.expect(TokenKind::Eof)?;

        Ok(expr)
    }

    fn parse_expr(&mut self) -> Result<Box<Expr>, Error> {
        let mut left = self.parse_term()?;

        while let Some(op) = self.accept_any(&[TokenKind::Plus, TokenKind::Minus]) {
            let right = self.parse_term()?;

            left = Expr::binary(BinaryOp::from_token(op), left, right);
        }

        Ok(left)
    }

    fn parse_term(&mut self) -> Result<Box<Expr>, Error> {
        let mut left = self.parse_factor()?;

        while let Some(op) = self.accept_any(&[TokenKind::Star, TokenKind::Slash]) {
            let right = self.parse_factor()?;

            left = Expr::binary(BinaryOp::from_token(op), left, right);
        }

        Ok(left)
    }

    fn parse_factor(&mut self) -> Result<Box<Expr>, Error> {
        match self.current().kind() {
            TokenKind::Int => {
                let int = self.advance();

                Ok(Expr::int(int.int_value()))
            }

            TokenKind::LParen => {
                self.advance();
                let expr = self.parse_expr()?;
                self.expect(TokenKind::RParen)?;

                Ok(expr)
            }

            _ => Err(Error::new(format!(
                "expected {} or {}",
                TokenKind::Int,
                TokenKind::LParen,
            ))),
        }
    }

    fn accept_any(&mut self, kinds: &[TokenKind]) -> Option<Token> {
        if kinds.contains(&self.current().kind()) {
            Some(self.advance())
        } else {
            None
        }
    }

    fn expect(&mut self, kind: TokenKind) -> Result<Token, Error> {
        if self.current().kind() == kind {
            Ok(self.advance())
        } else {
            Err(Error::new(format!("expected {kind}")))
        }
    }

    fn advance(&mut self) -> Token {
        mem::replace(&mut self.current, self.scanner.next())
    }

    fn current(&self) -> &Token {
        &self.current
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! assert_parses {
        ($input:expr, $ast:expr $(,)?) => {
            let mut parser = Parser::new($input);

            assert_eq!(parser.parse(), Ok($ast));
        };
    }

    macro_rules! assert_does_not_parse {
        ($input:expr, $error:expr $(,)?) => {
            let mut parser = Parser::new($input);

            assert_eq!(parser.parse(), Err(Error::new($error)));
        };
    }

    // Canonical expr is `1 + 2`.
    #[test]
    fn parses_expr() {
        assert_parses!(
            "1 * 2",
            Expr::binary(BinaryOp::Mul, Expr::int(1), Expr::int(2)),
        );
        assert_parses!(
            "1 * 2 + 3 * 4",
            Expr::binary(
                BinaryOp::Add,
                Expr::binary(BinaryOp::Mul, Expr::int(1), Expr::int(2)),
                Expr::binary(BinaryOp::Mul, Expr::int(3), Expr::int(4)),
            ),
        );
        assert_parses!(
            "1 * 2 - 3 * 4",
            Expr::binary(
                BinaryOp::Sub,
                Expr::binary(BinaryOp::Mul, Expr::int(1), Expr::int(2)),
                Expr::binary(BinaryOp::Mul, Expr::int(3), Expr::int(4)),
            ),
        );
        assert_parses!(
            "1 * 2 + 3 * 4 + 5 * 6 + 7 * 8",
            Expr::binary(
                BinaryOp::Add,
                Expr::binary(
                    BinaryOp::Add,
                    Expr::binary(
                        BinaryOp::Add,
                        Expr::binary(BinaryOp::Mul, Expr::int(1), Expr::int(2)),
                        Expr::binary(BinaryOp::Mul, Expr::int(3), Expr::int(4)),
                    ),
                    Expr::binary(BinaryOp::Mul, Expr::int(5), Expr::int(6)),
                ),
                Expr::binary(BinaryOp::Mul, Expr::int(7), Expr::int(8)),
            ),
        );

        // errors
        assert_does_not_parse!("", "expected integer literal or `(`");
        assert_does_not_parse!("%", "expected integer literal or `(`");
        assert_does_not_parse!("1 + ", "expected integer literal or `(`");
        assert_does_not_parse!("1 + %", "expected integer literal or `(`");
    }

    // Canonical term is `1 * 2`.
    #[test]
    fn parses_term() {
        assert_parses!("1", Expr::int(1));
        assert_parses!(
            "1 * 2",
            Expr::binary(BinaryOp::Mul, Expr::int(1), Expr::int(2)),
        );
        assert_parses!(
            "1 / 2",
            Expr::binary(BinaryOp::Div, Expr::int(1), Expr::int(2)),
        );
        assert_parses!(
            "1 * 2 * 3 * 4",
            Expr::binary(
                BinaryOp::Mul,
                Expr::binary(
                    BinaryOp::Mul,
                    Expr::binary(BinaryOp::Mul, Expr::int(1), Expr::int(2)),
                    Expr::int(3),
                ),
                Expr::int(4),
            ),
        );

        // errors
        assert_does_not_parse!("", "expected integer literal or `(`");
        assert_does_not_parse!("%", "expected integer literal or `(`");
        assert_does_not_parse!("1 * ", "expected integer literal or `(`");
        assert_does_not_parse!("1 * %", "expected integer literal or `(`");
    }

    // Canonical factor is `1`.
    #[test]
    fn parses_factor() {
        assert_parses!("1", Expr::int(1));
        assert_parses!(
            "(1 + 2)",
            Expr::binary(BinaryOp::Add, Expr::int(1), Expr::int(2)),
        );

        // errors
        assert_does_not_parse!("(", "expected integer literal or `(`");
        assert_does_not_parse!("(%", "expected integer literal or `(`");
        assert_does_not_parse!("(1 + 2", "expected `)`");
        assert_does_not_parse!("(1 + 2%", "expected `)`");
    }

    #[test]
    fn parses_complex_expressions() {
        assert_parses!(
            "(1 + 2) * (3 + 4)",
            Expr::binary(
                BinaryOp::Mul,
                Expr::binary(BinaryOp::Add, Expr::int(1), Expr::int(2)),
                Expr::binary(BinaryOp::Add, Expr::int(3), Expr::int(4)),
            ),
        );
    }

    #[test]
    fn does_not_parse_empty_input() {
        assert_does_not_parse!("", "expected integer literal or `(`");
    }

    #[test]
    fn does_not_parse_trailing_input() {
        assert_does_not_parse!("1 + 2%", "expected end of input");
    }
}
