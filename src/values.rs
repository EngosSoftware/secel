//!

use rust_decimal::Decimal;
use std::fmt;

/// Value definition.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Value {
  /// Value representing a `NULL`.
  Null,
  /// Value representing a boolean.
  Bool(bool),
  /// Value representing a decimal number.
  Number(Decimal),
}

impl fmt::Display for Value {
  /// Implements [Display](std::fmt::Display) for [Value].
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Value::Null => write!(f, "Null"),
      Value::Bool(v) => write!(f, "Bool: {}", v),
      Value::Number(v) => write!(f, "Number: {}", v),
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use rust_decimal::Decimal;

  #[test]
  fn test_display() {
    assert_eq!("Null", format!("{}", Value::Null));
    assert_eq!("Bool: true", format!("{}", Value::Bool(true)));
    assert_eq!("Bool: false", format!("{}", Value::Bool(false)));
    assert_eq!("Number: 1.11", format!("{}", Value::Number(Decimal::new(111, 2))));
  }

  #[test]
  fn test_debug() {
    assert_eq!("Null", format!("{:?}", Value::Null));
    assert_eq!("Bool(true)", format!("{:?}", Value::Bool(true)));
    assert_eq!("Bool(false)", format!("{:?}", Value::Bool(false)));
    let n = Decimal::new(111, 2);
    assert_eq!("Number(1.11)", format!("{:?}", Value::Number(n)));
  }

  #[test]
  #[allow(clippy::clone_on_copy)]
  fn test_comparison() {
    assert!((Value::Null == Value::Null));
    assert!((Value::Bool(true) == Value::Bool(true)));
    assert!((Value::Bool(true) != Value::Bool(false)));
    let n1 = Decimal::new(111, 2);
    let n2 = Decimal::new(222, 2);
    assert!((Value::Number(n1) == Value::Number(n1)));
    assert!((Value::Number(n1) != Value::Number(n2)));
    assert!((Value::Number(n1).clone() != Value::Number(n2).clone()));
  }
}
