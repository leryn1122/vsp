use crate::span::Span;

pub mod span;

/// A trait for objects, such as lexeme, token, or something else, give the span itself.
pub trait Locatable {
  fn get_span(&self) -> &Span;
}
