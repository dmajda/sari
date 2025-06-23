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
        while let Some(&ch) = self.chars.peek() {
            if ch == ' ' || ch == '\t' || ch == '\r' || ch == '\n' {
                self.chars.next();
            } else {
                break;
            }
        }
    }

    fn scan_int_rest(&mut self, first_ch: char) -> Token {
        // There is `char::to_digit`, but it's too complicated because it
        // supports bases other than 10 and it doesn't assume a valid digit,
        // which means it returns an option.
        //
        // Let's roll out our own, simpler version.
        fn to_digit(ch: char) -> i32 {
            (ch as u32).wrapping_sub('0' as u32) as i32
        }

        let mut value = to_digit(first_ch);

        while let Some(&ch) = self.chars.peek() {
            if ch.is_ascii_digit() {
                value = value.wrapping_mul(10).wrapping_add(to_digit(ch));

                self.chars.next();
            } else {
                break;
            }
        }

        Token::Int(value)
    }
}

impl Iterator for Scanner<'_> {
    type Item = Token;

    fn next(&mut self) -> Option<Token> {
        self.skip_whitespace();

        let ch = self.chars.next()?;
        let token = match ch {
            '+' => Token::Plus,
            '-' => Token::Minus,
            '*' => Token::Star,
            '/' => Token::Slash,
            '(' => Token::LParen,
            ')' => Token::RParen,

            '0'..='9' => self.scan_int_rest(ch),

            _ => Token::Error(ch),
        };

        Some(token)
    }
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
        assert_scans!(" 1", vec![Token::Int(1)]);
        assert_scans!("\t1", vec![Token::Int(1)]);
        assert_scans!("\r1", vec![Token::Int(1)]);
        assert_scans!("\n1", vec![Token::Int(1)]);
        assert_scans!("   1", vec![Token::Int(1)]);

        // after
        assert_scans!("1 ", vec![Token::Int(1)]);
        assert_scans!("1\t", vec![Token::Int(1)]);
        assert_scans!("1\r", vec![Token::Int(1)]);
        assert_scans!("1\n", vec![Token::Int(1)]);
        assert_scans!("1   ", vec![Token::Int(1)]);
    }

    #[test]
    fn scans_simple_tokens() {
        assert_scans!("+", vec![Token::Plus]);
        assert_scans!("-", vec![Token::Minus]);
        assert_scans!("*", vec![Token::Star]);
        assert_scans!("/", vec![Token::Slash]);
        assert_scans!("(", vec![Token::LParen]);
        assert_scans!(")", vec![Token::RParen]);
    }

    #[test]
    fn scans_int_token() {
        assert_scans!("0", vec![Token::Int(0)]);
        assert_scans!("9", vec![Token::Int(9)]);
        assert_scans!("123", vec![Token::Int(123)]);

        // overflow
        assert_scans!("2147483647", vec![Token::Int(2147483647)]);
        assert_scans!("2147483648", vec![Token::Int(-2147483648)]);
    }

    #[test]
    fn scans_error_token() {
        assert_scans!("%", vec![Token::Error('%')]);
    }

    #[test]
    fn scans_multiple_tokens() {
        assert_scans!("1+2", vec![Token::Int(1), Token::Plus, Token::Int(2)]);
    }
}
