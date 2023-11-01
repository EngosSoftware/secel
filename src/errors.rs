//! Errors implementation.

use std::fmt;

/// Common result type.
pub type Result<T, E = SecelError> = std::result::Result<T, E>;

/// Common error definition.
#[derive(Debug, PartialEq, Eq)]
pub struct SecelError(String);

impl fmt::Display for SecelError {
  /// Implementation of [Display](std::fmt::Display) trait for [SecelError].
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", self.0)
  }
}

impl SecelError {
  /// Creates a new [SecelError] with specified message text.
  pub fn new(message: &str) -> Self {
    Self(message.to_string())
  }
}
