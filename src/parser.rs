use std::cell::RefCell;
use std::mem;
use std::rc::Rc;

use crate::ast::{BinaryOp, Expr};
use crate::error::Error;
use crate::scanner::Scanner;
use crate::source::{SourceMap, SourceSpan, Span, Spanned};
use crate::token::{Token, TokenKind};

pub struct Parser<'a> {
    scanner: Scanner<'a>,
    source_map: Rc<RefCell<SourceMap>>,
    current: Token,
}

impl Parser<'_> {
    pub fn new(input: &str, source_map: Rc<RefCell<SourceMap>>) -> Parser {
        Parser {
            scanner: Scanner::new(input, Rc::clone(&source_map)),
            source_map,
            current: Token::eof(Span::new(0, 0)),
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
            let span = Span::cover(left.span(), right.span());

            left = Expr::binary(span, BinaryOp::from_token(op), left, right);
        }

        Ok(left)
    }

    fn parse_term(&mut self) -> Result<Box<Expr>, Error> {
        let mut left = self.parse_factor()?;

        while let Some(op) = self.accept_any(&[TokenKind::Star, TokenKind::Slash]) {
            let right = self.parse_factor()?;
            let span = Span::cover(left.span(), right.span());

            left = Expr::binary(span, BinaryOp::from_token(op), left, right);
        }

        Ok(left)
    }

    fn parse_factor(&mut self) -> Result<Box<Expr>, Error> {
        match self.current().kind() {
            TokenKind::Int => {
                let int = self.advance();

                Ok(Expr::int(int.span(), int.int_value()))
            }

            TokenKind::LParen => {
                let l_paren = self.advance();
                let expr = self.parse_expr()?;
                let r_paren = self.expect(TokenKind::RParen)?;
                let span = Span::cover(l_paren.span(), r_paren.span());

                Ok(Expr::group(span, expr))
            }

            _ => Err(self.error(
                self.current(),
                format!("expected {} or {}", TokenKind::Int, TokenKind::LParen),
            )),
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
            Err(self.error(self.current(), format!("expected {kind}")))
        }
    }

    fn advance(&mut self) -> Token {
        mem::replace(&mut self.current, self.scanner.scan())
    }

    fn current(&self) -> &Token {
        &self.current
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
    use crate::{SourcePos, SourceSpan};

    macro_rules! assert_parses {
        ($input:expr, $ast:expr $(,)?) => {
            let source_map = Rc::new(RefCell::new(SourceMap::new()));
            let mut parser = Parser::new($input, Rc::clone(&source_map));

            assert_eq!(parser.parse(), Ok($ast));
        };
    }

    macro_rules! assert_does_not_parse {
        ($input:expr, $error:expr $(,)?) => {
            let source_map = Rc::new(RefCell::new(SourceMap::new()));
            let mut parser = Parser::new($input, Rc::clone(&source_map));

            assert_eq!(parser.parse(), Err($error));
        };
    }

    // Canonical expr is `1 + 2`.
    #[test]
    fn parses_expr() {
        assert_parses!(
            "1 * 2",
            Expr::binary(
                Span::new(0, 5),
                BinaryOp::Mul,
                Expr::int(Span::new(0, 1), 1),
                Expr::int(Span::new(4, 5), 2),
            ),
        );
        assert_parses!(
            "1 * 2 + 3 * 4",
            Expr::binary(
                Span::new(0, 13),
                BinaryOp::Add,
                Expr::binary(
                    Span::new(0, 5),
                    BinaryOp::Mul,
                    Expr::int(Span::new(0, 1), 1),
                    Expr::int(Span::new(4, 5), 2),
                ),
                Expr::binary(
                    Span::new(8, 13),
                    BinaryOp::Mul,
                    Expr::int(Span::new(8, 9), 3),
                    Expr::int(Span::new(12, 13), 4),
                ),
            ),
        );
        assert_parses!(
            "1 * 2 - 3 * 4",
            Expr::binary(
                Span::new(0, 13),
                BinaryOp::Sub,
                Expr::binary(
                    Span::new(0, 5),
                    BinaryOp::Mul,
                    Expr::int(Span::new(0, 1), 1),
                    Expr::int(Span::new(4, 5), 2),
                ),
                Expr::binary(
                    Span::new(8, 13),
                    BinaryOp::Mul,
                    Expr::int(Span::new(8, 9), 3),
                    Expr::int(Span::new(12, 13), 4),
                ),
            ),
        );
        assert_parses!(
            "1 * 2 + 3 * 4 + 5 * 6 + 7 * 8",
            Expr::binary(
                Span::new(0, 29),
                BinaryOp::Add,
                Expr::binary(
                    Span::new(0, 21),
                    BinaryOp::Add,
                    Expr::binary(
                        Span::new(0, 13),
                        BinaryOp::Add,
                        Expr::binary(
                            Span::new(0, 5),
                            BinaryOp::Mul,
                            Expr::int(Span::new(0, 1), 1),
                            Expr::int(Span::new(4, 5), 2),
                        ),
                        Expr::binary(
                            Span::new(8, 13),
                            BinaryOp::Mul,
                            Expr::int(Span::new(8, 9), 3),
                            Expr::int(Span::new(12, 13), 4),
                        ),
                    ),
                    Expr::binary(
                        Span::new(16, 21),
                        BinaryOp::Mul,
                        Expr::int(Span::new(16, 17), 5),
                        Expr::int(Span::new(20, 21), 6),
                    ),
                ),
                Expr::binary(
                    Span::new(24, 29),
                    BinaryOp::Mul,
                    Expr::int(Span::new(24, 25), 7),
                    Expr::int(Span::new(28, 29), 8),
                ),
            ),
        );

        // errors
        assert_does_not_parse!(
            "",
            Error::new(
                SourceSpan::new(SourcePos::new(0, 1, 1), SourcePos::new(0, 1, 1)),
                "expected integer literal or `(`",
            ),
        );
        assert_does_not_parse!(
            "%",
            Error::new(
                SourceSpan::new(SourcePos::new(0, 1, 1), SourcePos::new(1, 1, 2)),
                "expected integer literal or `(`",
            ),
        );
        assert_does_not_parse!(
            "1 + ",
            Error::new(
                SourceSpan::new(SourcePos::new(4, 1, 5), SourcePos::new(4, 1, 5)),
                "expected integer literal or `(`",
            ),
        );
        assert_does_not_parse!(
            "1 + %",
            Error::new(
                SourceSpan::new(SourcePos::new(4, 1, 5), SourcePos::new(5, 1, 6)),
                "expected integer literal or `(`",
            ),
        );
    }

    // Canonical term is `1 * 2`.
    #[test]
    fn parses_term() {
        assert_parses!("1", Expr::int(Span::new(0, 1), 1));
        assert_parses!(
            "1 * 2",
            Expr::binary(
                Span::new(0, 5),
                BinaryOp::Mul,
                Expr::int(Span::new(0, 1), 1),
                Expr::int(Span::new(4, 5), 2),
            ),
        );
        assert_parses!(
            "1 / 2",
            Expr::binary(
                Span::new(0, 5),
                BinaryOp::Div,
                Expr::int(Span::new(0, 1), 1),
                Expr::int(Span::new(4, 5), 2),
            ),
        );
        assert_parses!(
            "1 * 2 * 3 * 4",
            Expr::binary(
                Span::new(0, 13),
                BinaryOp::Mul,
                Expr::binary(
                    Span::new(0, 9),
                    BinaryOp::Mul,
                    Expr::binary(
                        Span::new(0, 5),
                        BinaryOp::Mul,
                        Expr::int(Span::new(0, 1), 1),
                        Expr::int(Span::new(4, 5), 2),
                    ),
                    Expr::int(Span::new(8, 9), 3),
                ),
                Expr::int(Span::new(12, 13), 4),
            ),
        );

        // errors
        assert_does_not_parse!(
            "",
            Error::new(
                SourceSpan::new(SourcePos::new(0, 1, 1), SourcePos::new(0, 1, 1)),
                "expected integer literal or `(`",
            ),
        );
        assert_does_not_parse!(
            "%",
            Error::new(
                SourceSpan::new(SourcePos::new(0, 1, 1), SourcePos::new(1, 1, 2)),
                "expected integer literal or `(`",
            ),
        );
        assert_does_not_parse!(
            "1 * ",
            Error::new(
                SourceSpan::new(SourcePos::new(4, 1, 5), SourcePos::new(4, 1, 5)),
                "expected integer literal or `(`",
            ),
        );
        assert_does_not_parse!(
            "1 * %",
            Error::new(
                SourceSpan::new(SourcePos::new(4, 1, 5), SourcePos::new(5, 1, 6)),
                "expected integer literal or `(`",
            ),
        );
    }

    // Canonical factor is `1`.
    #[test]
    fn parses_factor() {
        assert_parses!("1", Expr::int(Span::new(0, 1), 1));
        assert_parses!(
            "(1 + 2)",
            Expr::group(
                Span::new(0, 7),
                Expr::binary(
                    Span::new(1, 6),
                    BinaryOp::Add,
                    Expr::int(Span::new(1, 2), 1),
                    Expr::int(Span::new(5, 6), 2),
                ),
            ),
        );

        // errors
        assert_does_not_parse!(
            "(",
            Error::new(
                SourceSpan::new(SourcePos::new(1, 1, 2), SourcePos::new(1, 1, 2)),
                "expected integer literal or `(`",
            ),
        );
        assert_does_not_parse!(
            "(%",
            Error::new(
                SourceSpan::new(SourcePos::new(1, 1, 2), SourcePos::new(2, 1, 3)),
                "expected integer literal or `(`",
            ),
        );
        assert_does_not_parse!(
            "(1 + 2",
            Error::new(
                SourceSpan::new(SourcePos::new(6, 1, 7), SourcePos::new(6, 1, 7)),
                "expected `)`",
            ),
        );
        assert_does_not_parse!(
            "(1 + 2%",
            Error::new(
                SourceSpan::new(SourcePos::new(6, 1, 7), SourcePos::new(7, 1, 8)),
                "expected `)`",
            ),
        );
    }

    #[test]
    fn parses_complex_expressions() {
        assert_parses!(
            "(1 + 2) * (3 + 4)",
            Expr::binary(
                Span::new(0, 17),
                BinaryOp::Mul,
                Expr::group(
                    Span::new(0, 7),
                    Expr::binary(
                        Span::new(1, 6),
                        BinaryOp::Add,
                        Expr::int(Span::new(1, 2), 1),
                        Expr::int(Span::new(5, 6), 2),
                    ),
                ),
                Expr::group(
                    Span::new(10, 17),
                    Expr::binary(
                        Span::new(11, 16),
                        BinaryOp::Add,
                        Expr::int(Span::new(11, 12), 3),
                        Expr::int(Span::new(15, 16), 4),
                    ),
                ),
            ),
        );
    }

    #[test]
    fn does_not_parse_empty_input() {
        assert_does_not_parse!(
            "",
            Error::new(
                SourceSpan::new(SourcePos::new(0, 1, 1), SourcePos::new(0, 1, 1)),
                "expected integer literal or `(`",
            ),
        );
    }

    #[test]
    fn does_not_parse_trailing_input() {
        assert_does_not_parse!(
            "1 + 2%",
            Error::new(
                SourceSpan::new(SourcePos::new(5, 1, 6), SourcePos::new(6, 1, 7)),
                "expected end of input",
            ),
        );
    }
}
