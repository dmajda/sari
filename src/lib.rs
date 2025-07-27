#![warn(missing_docs)]

//! Simple arithmetic expression evaluator.
//!
//! # Usage
//!
//! To evaluate an expression, use the `sari::eval` function:
//!
//! ```
//! use sari::Error;
//!
//! let result = sari::eval("(1 + 2) * 3");
//! assert_eq!(result, Ok(9));
//!
//! let result = sari::eval("(1 + 2");
//! assert_eq!(result, Err(Error::new("expected `)`")));
//!
//! let result = sari::eval("1 / 0");
//! assert_eq!(result, Err(Error::new("division by zero")));
//! ```
//!
//! # Expressions
//!
//! The expressions consist of integers combined using `+`, `-`, `*`, and `/`
//! binary operators (with the usual precedence and associativity) and grouped
//! using parentheses. These elements can be separated by whitespace.
//!
//! The expressions use wrapping 32-bit signed arithmetic. Division by zero is
//! an error.

mod ast;
mod error;
mod evaluator;
mod parser;
mod scanner;
mod token;

#[doc(inline)]
pub use error::Error;

use evaluator::Evaluator;
use parser::Parser;

/// Evaluates an expression and returns the result.
///
/// # Errors
///
/// Returns [`Error`] if the evaluation fails.
///
/// # Examples
///
/// ```
/// use sari::Error;
///
/// let result = sari::eval("(1 + 2) * 3");
/// assert_eq!(result, Ok(9));
///
/// let result = sari::eval("(1 + 2");
/// assert_eq!(result, Err(Error::new("expected `)`")));
///
/// let result = sari::eval("1 / 0");
/// assert_eq!(result, Err(Error::new("division by zero")));
/// ```
pub fn eval(expr: &str) -> Result<i32, Error> {
    let ast = Parser::new(expr).parse()?;
    let value = Evaluator::new(&ast).eval()?;

    Ok(value)
}
