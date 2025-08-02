use std::{error, fmt};

/// Error returned when expression evaluation fails.
///
/// # Examples
///
/// ```
/// use sari::Error;
///
/// let result = sari::eval("1 / 0");
/// assert_eq!(result, Err(Error::new("division by zero")));
/// ```
#[derive(Clone, Eq, PartialEq, Debug)]
pub struct Error {
    message: String,
}

impl Error {
    /// Creates a new error with specified message.
    ///
    /// # Examples
    ///
    /// ```
    /// use sari::Error;
    ///
    /// let error = Error::new("division by zero");
    /// ```
    pub fn new(message: impl Into<String>) -> Error {
        Error {
            message: message.into(),
        }
    }

    /// Returns the message.
    ///
    /// # Examples
    ///
    /// ```
    /// use sari::Error;
    ///
    /// let error = Error::new("division by zero");
    /// assert_eq!(error.message(), "division by zero");
    /// ```
    pub fn message(&self) -> &str {
        &self.message
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", &self.message)
    }
}

impl error::Error for Error {}
