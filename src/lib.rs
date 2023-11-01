mod ast;
mod errors;
mod evaluator;
mod lexer;
mod parser;
mod values;

#[cfg(test)]
mod tests;

pub use ast::AstNode;
pub use evaluator::{Evaluator, IndexKey, IndexedValues};
pub use values::Value;

/// Parses expression, panics on failure.
pub fn parse_expression(input: &str) -> AstNode {
  parser::Parser::new(input).parse().unwrap()
}

/// Builds evaluator, panics on failure.
pub fn build_evaluator(input: &str) -> Evaluator {
  let node = parser::Parser::new(input).parse().unwrap();
  evaluator::build_evaluator(&node).unwrap()
}
