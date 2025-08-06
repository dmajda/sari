use std::{error, fmt};

use crate::SourceSpan;

/// Error returned when expression evaluation fails.
///
/// # Examples
///
/// ```
/// use sari::{Error, SourcePos, SourceSpan};
///
/// let result = sari::eval("1 / 0");
///
/// let span = SourceSpan::new(
///     SourcePos::new(0, 1, 1), // offset 0, line 1, column 1
///     SourcePos::new(5, 1, 6), // offset 5, line 1, column 6
/// );
/// let error = Error::new(span, "division by zero");
///
/// assert_eq!(result, Err(error));
/// ```
#[derive(Clone, Eq, PartialEq, Debug)]
pub struct Error {
    span: SourceSpan,
    message: String,
}

impl Error {
    /// Creates a new `Error` with specified span and message.
    ///
    /// # Examples
    ///
    /// ```
    /// use sari::{Error, SourcePos, SourceSpan};
    ///
    /// let span = SourceSpan::new(
    ///     SourcePos::new(69, 5, 7), // offset 69, line 5, column 7
    ///     SourcePos::new(74, 5, 12), // offset 74, line 5, column 12
    /// );
    /// let error = Error::new(span, "division by zero");
    ///
    /// assert_eq!(error.span(), span);
    /// assert_eq!(error.message(), "division by zero");
    /// ```
    pub fn new(span: SourceSpan, message: impl Into<String>) -> Error {
        Error {
            span,
            message: message.into(),
        }
    }

    /// Returns the span.
    pub fn span(&self) -> SourceSpan {
        self.span
    }

    /// Returns the message.
    pub fn message(&self) -> &str {
        &self.message
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", &self.span, &self.message)
    }
}

impl error::Error for Error {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::SourcePos;

    #[test]
    fn error_fmt_works() {
        let span = SourceSpan::new(SourcePos::new(4, 1, 5), SourcePos::new(8, 2, 3));
        let error = Error::new(span, "division by zero");

        assert_eq!(error.to_string(), "1:5-2:3: division by zero");
    }
}
