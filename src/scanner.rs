use std::cell::RefCell;
use std::iter::Peekable;
use std::rc::Rc;
use std::str::Chars;

use crate::source::{SourceMap, Span};
use crate::token::Token;

pub struct Scanner<'a> {
    chars: Peekable<Chars<'a>>,
    source_map: Rc<RefCell<SourceMap<'a>>>,
    pos: usize,
    start_pos: usize,
}

impl Scanner<'_> {
    pub fn new<'a>(input: &'a str, source_map: Rc<RefCell<SourceMap<'a>>>) -> Scanner<'a> {
        Scanner {
            chars: input.chars().peekable(),
            source_map,
            pos: 0,
            start_pos: 0,
        }
    }

    pub fn scan(&mut self) -> Token {
        self.skip_whitespace();
        self.start();

        let Some(ch) = self.next() else {
            return Token::eof(self.span());
        };

        match ch {
            '+' => Token::plus(self.span()),
            '-' => Token::minus(self.span()),
            '*' => Token::star(self.span()),
            '/' => Token::slash(self.span()),
            '(' => Token::l_paren(self.span()),
            ')' => Token::r_paren(self.span()),

            '0'..='9' => self.scan_int_rest(ch),

            _ => Token::error(self.span()),
        }
    }

    fn skip_whitespace(&mut self) {
        while let Some(&ch) = self.peek()
            && is_whitespace(ch)
        {
            self.next();
        }
    }

    fn scan_int_rest(&mut self, first_ch: char) -> Token {
        let mut value = to_digit(first_ch);

        while let Some(&ch) = self.peek()
            && is_digit(ch)
        {
            self.next();

            value = value.wrapping_mul(10).wrapping_add(to_digit(ch));
        }

        Token::int(self.span(), value)
    }

    fn start(&mut self) {
        self.start_pos = self.pos;
    }

    fn peek(&mut self) -> Option<&char> {
        self.chars.peek()
    }

    fn next(&mut self) -> Option<char> {
        self.chars.next().inspect(|&ch| {
            self.pos += ch.len_utf8();

            if ch == '\n' {
                self.add_line_start(self.pos);
            }
        })
    }

    fn add_line_start(&self, pos: usize) {
        self.source_map.borrow_mut().add_line_start(pos)
    }

    fn span(&mut self) -> Span {
        Span::new(self.start_pos, self.pos)
    }
}

fn is_whitespace(ch: char) -> bool {
    ch == ' ' || ch == '\t' || ch == '\r' || ch == '\n'
}

fn is_digit(ch: char) -> bool {
    ch.is_ascii_digit()
}

fn to_digit(ch: char) -> i32 {
    (ch as u32).wrapping_sub('0' as u32) as i32
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::source::{SourcePos, SourceSpan};
    use crate::token::TokenKind;

    macro_rules! assert_scans {
        ($input:expr, $tokens:expr $(,)?) => {
            let source_map = Rc::new(RefCell::new(SourceMap::new($input)));
            let mut scanner = Scanner::new($input, Rc::clone(&source_map));

            let mut tokens = vec![];
            while let token = scanner.scan()
                && token.kind() != TokenKind::Eof
            {
                tokens.push(token);
            }

            assert_eq!(tokens, $tokens);
        };
    }

    #[test]
    fn scans_empty_input() {
        assert_scans!("", vec![]);
    }

    #[test]
    fn skips_whitespace() {
        // before
        assert_scans!(" 1", vec![Token::int(Span::new(1, 2), 1)]);
        assert_scans!("\t1", vec![Token::int(Span::new(1, 2), 1)]);
        assert_scans!("\r1", vec![Token::int(Span::new(1, 2), 1)]);
        assert_scans!("\n1", vec![Token::int(Span::new(1, 2), 1)]);
        assert_scans!("   1", vec![Token::int(Span::new(3, 4), 1)]);

        // after
        assert_scans!("1 ", vec![Token::int(Span::new(0, 1), 1)]);
        assert_scans!("1\t", vec![Token::int(Span::new(0, 1), 1)]);
        assert_scans!("1\r", vec![Token::int(Span::new(0, 1), 1)]);
        assert_scans!("1\n", vec![Token::int(Span::new(0, 1), 1)]);
        assert_scans!("1   ", vec![Token::int(Span::new(0, 1), 1)]);
    }

    #[test]
    fn scans_simple_tokens() {
        assert_scans!("+", vec![Token::plus(Span::new(0, 1))]);
        assert_scans!("-", vec![Token::minus(Span::new(0, 1))]);
        assert_scans!("*", vec![Token::star(Span::new(0, 1))]);
        assert_scans!("/", vec![Token::slash(Span::new(0, 1))]);
        assert_scans!("(", vec![Token::l_paren(Span::new(0, 1))]);
        assert_scans!(")", vec![Token::r_paren(Span::new(0, 1))]);
    }

    #[test]
    fn scans_int_token() {
        assert_scans!("0", vec![Token::int(Span::new(0, 1), 0)]);
        assert_scans!("9", vec![Token::int(Span::new(0, 1), 9)]);
        assert_scans!("123", vec![Token::int(Span::new(0, 3), 123)]);

        // overflow
        assert_scans!("2147483647", vec![Token::int(Span::new(0, 10), 2147483647)]);
        assert_scans!(
            "2147483648",
            vec![Token::int(Span::new(0, 10), -2147483648)],
        );
    }

    #[test]
    fn scans_error_token() {
        assert_scans!("%", vec![Token::error(Span::new(0, 1))]);

        // Unicode
        assert_scans!("‰", vec![Token::error(Span::new(0, 3))]);
    }

    #[test]
    fn scans_multiple_tokens() {
        assert_scans!(
            "1+2",
            vec![
                Token::int(Span::new(0, 1), 1),
                Token::plus(Span::new(1, 2)),
                Token::int(Span::new(2, 3), 2),
            ],
        );
    }

    #[test]
    fn updates_source_map() {
        let input = "1 +\n‰ +\n3";
        let source_map = Rc::new(RefCell::new(SourceMap::new(input)));
        let mut scanner = Scanner::new(input, Rc::clone(&source_map));

        while scanner.scan().kind() != TokenKind::Eof {}

        let source_map = source_map.borrow();

        // line 1
        assert_eq!(
            source_map.map_span(Span::new(0, 1)),
            SourceSpan::new(SourcePos::new(0, 1, 1), SourcePos::new(1, 1, 2))
        );

        // line 2
        assert_eq!(
            source_map.map_span(Span::new(4, 7)),
            SourceSpan::new(SourcePos::new(4, 2, 1), SourcePos::new(7, 2, 2))
        );

        // line 3
        assert_eq!(
            source_map.map_span(Span::new(10, 11)),
            SourceSpan::new(SourcePos::new(10, 3, 1), SourcePos::new(11, 3, 2))
        );
    }
}
