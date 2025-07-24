use std::{iter::Peekable, str::Chars};

use crate::token::Token;

pub struct Scanner<'a> {
    chars: Peekable<Chars<'a>>,
}

impl Scanner<'_> {
    pub fn new(input: &str) -> Scanner {
        Scanner {
            chars: input.chars().peekable(),
        }
    }

    fn skip_whitespace(&mut self) {
        while let Some(&ch) = self.chars.peek()
            && is_whitespace(ch)
        {
            self.chars.next();
        }
    }

    fn scan_int_rest(&mut self, first_ch: char) -> Token {
        let mut value = to_digit(first_ch);

        while let Some(&ch) = self.chars.peek()
            && is_digit(ch)
        {
            self.chars.next();

            value = value.wrapping_mul(10).wrapping_add(to_digit(ch));
        }

        Token::int(value)
    }
}

impl Iterator for Scanner<'_> {
    type Item = Token;

    fn next(&mut self) -> Option<Token> {
        self.skip_whitespace();

        let ch = self.chars.next()?;
        let token = match ch {
            '+' => Token::plus(),
            '-' => Token::minus(),
            '*' => Token::star(),
            '/' => Token::slash(),
            '(' => Token::l_paren(),
            ')' => Token::r_paren(),

            '0'..='9' => self.scan_int_rest(ch),

            _ => Token::error(),
        };

        Some(token)
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

    macro_rules! assert_scans {
        ($input:expr, $tokens:expr) => {
            let scanner = Scanner::new($input);

            assert_eq!(scanner.collect::<Vec<_>>(), $tokens);
        };
    }

    #[test]
    fn scans_empty_input() {
        assert_scans!("", vec![]);
    }

    #[test]
    fn skips_whitespace() {
        // before
        assert_scans!(" 1", vec![Token::int(1)]);
        assert_scans!("\t1", vec![Token::int(1)]);
        assert_scans!("\r1", vec![Token::int(1)]);
        assert_scans!("\n1", vec![Token::int(1)]);
        assert_scans!("   1", vec![Token::int(1)]);

        // after
        assert_scans!("1 ", vec![Token::int(1)]);
        assert_scans!("1\t", vec![Token::int(1)]);
        assert_scans!("1\r", vec![Token::int(1)]);
        assert_scans!("1\n", vec![Token::int(1)]);
        assert_scans!("1   ", vec![Token::int(1)]);
    }

    #[test]
    fn scans_simple_tokens() {
        assert_scans!("+", vec![Token::plus()]);
        assert_scans!("-", vec![Token::minus()]);
        assert_scans!("*", vec![Token::star()]);
        assert_scans!("/", vec![Token::slash()]);
        assert_scans!("(", vec![Token::l_paren()]);
        assert_scans!(")", vec![Token::r_paren()]);
    }

    #[test]
    fn scans_int_token() {
        assert_scans!("0", vec![Token::int(0)]);
        assert_scans!("9", vec![Token::int(9)]);
        assert_scans!("123", vec![Token::int(123)]);

        // overflow
        assert_scans!("2147483647", vec![Token::int(2147483647)]);
        assert_scans!("2147483648", vec![Token::int(-2147483648)]);
    }

    #[test]
    fn scans_error_token() {
        assert_scans!("%", vec![Token::error()]);

        // Unicode
        assert_scans!("‰", vec![Token::error()]);
    }

    #[test]
    fn scans_multiple_tokens() {
        assert_scans!("1+2", vec![Token::int(1), Token::plus(), Token::int(2)]);
    }
}
