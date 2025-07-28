use crate::ast::{BinaryOp, Expr};
use crate::error::Error;
use crate::scanner::Scanner;
use crate::token::{Token, TokenKind};

pub struct Parser<'a> {
    scanner: Scanner<'a>,
    peeked: Option<Token>,
}

impl Parser<'_> {
    pub fn new(input: &str) -> Parser {
        Parser {
            scanner: Scanner::new(input),
            peeked: None,
        }
    }

    pub fn parse(&mut self) -> Result<Box<Expr>, Error> {
        let expr = self.parse_expr()?;

        if self.next().kind() != TokenKind::Eof {
            return Err(Error::new("expected end of input"));
        }

        Ok(expr)
    }

    fn parse_expr(&mut self) -> Result<Box<Expr>, Error> {
        let mut term = self.parse_term()?;

        while matches!(self.peek().kind(), TokenKind::Plus | TokenKind::Minus) {
            let op = self.next();
            let right = self.parse_term()?;

            term = Expr::binary(BinaryOp::from_token(op), term, right);
        }

        Ok(term)
    }

    fn parse_term(&mut self) -> Result<Box<Expr>, Error> {
        let mut term = self.parse_factor()?;

        while matches!(self.peek().kind(), TokenKind::Star | TokenKind::Slash) {
            let op = self.next();
            let right = self.parse_factor()?;

            term = Expr::binary(BinaryOp::from_token(op), term, right);
        }

        Ok(term)
    }

    fn parse_factor(&mut self) -> Result<Box<Expr>, Error> {
        let token = self.next();

        match token.kind() {
            TokenKind::Int => Ok(Expr::int(token.int_value())),
            TokenKind::LParen => {
                let expr = self.parse_expr()?;

                if !matches!(self.next().kind(), TokenKind::RParen) {
                    return Err(Error::new("expected `)`"));
                }

                Ok(expr)
            }
            _ => Err(Error::new("expected integer literal or `(`")),
        }
    }

    fn peek(&mut self) -> &Token {
        self.peeked.get_or_insert_with(|| self.scanner.next())
    }

    fn next(&mut self) -> Token {
        self.peeked.take().unwrap_or_else(|| self.scanner.next())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
        assert_parses!(
            "1 * 2",
            Expr::binary(BinaryOp::Mul, Expr::int(1), Expr::int(2))
        );
        assert_parses!(
            "1 * 2 + 3 * 4",
            Expr::binary(
                BinaryOp::Add,
                Expr::binary(BinaryOp::Mul, Expr::int(1), Expr::int(2)),
                Expr::binary(BinaryOp::Mul, Expr::int(3), Expr::int(4))
            )
        );
        assert_parses!(
            "1 * 2 - 3 * 4",
            Expr::binary(
                BinaryOp::Sub,
                Expr::binary(BinaryOp::Mul, Expr::int(1), Expr::int(2)),
                Expr::binary(BinaryOp::Mul, Expr::int(3), Expr::int(4))
            )
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
                        Expr::binary(BinaryOp::Mul, Expr::int(3), Expr::int(4))
                    ),
                    Expr::binary(BinaryOp::Mul, Expr::int(5), Expr::int(6))
                ),
                Expr::binary(BinaryOp::Mul, Expr::int(7), Expr::int(8))
            )
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
            Expr::binary(BinaryOp::Mul, Expr::int(1), Expr::int(2))
        );
        assert_parses!(
            "1 / 2",
            Expr::binary(BinaryOp::Div, Expr::int(1), Expr::int(2))
        );
        assert_parses!(
            "1 * 2 * 3 * 4",
            Expr::binary(
                BinaryOp::Mul,
                Expr::binary(
                    BinaryOp::Mul,
                    Expr::binary(BinaryOp::Mul, Expr::int(1), Expr::int(2)),
                    Expr::int(3)
                ),
                Expr::int(4)
            )
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
            Expr::binary(BinaryOp::Add, Expr::int(1), Expr::int(2))
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
                Expr::binary(BinaryOp::Add, Expr::int(3), Expr::int(4))
            )
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
