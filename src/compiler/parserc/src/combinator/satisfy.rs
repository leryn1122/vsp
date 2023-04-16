use crate::parser::ParseState;
use crate::parser::Parser;

pub struct Satisfy<F> {
  satisfy: F,
}

impl<F> Satisfy<F> {
  pub fn new(satisfy: F) -> Self {
    Self { satisfy: satisfy }
  }
}

pub fn satisfy<F>(f: F) -> Satisfy<F>
where
  F: Fn(&char) -> bool,
{
  Satisfy::new(f)
}

impl<'a, F> Parser<ParseState<'a>> for Satisfy<F>
where
  F: Fn(&char) -> bool,
{
  type Target = char;

  fn parse(&self, state: &mut ParseState<'a>) -> Option<Self::Target> {
    match state.next() {
      None => None,
      Some(ch) => {
        if (self.satisfy)(&ch) {
          Some(ch)
        } else {
          None
        }
      }
    }
  }
}
