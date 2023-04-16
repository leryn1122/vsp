use crate::parser::ParseState;
use crate::parser::Parser;

/// Parser for EOF as the terminator for reading a single file.
#[derive(Copy, Clone, Debug)]
pub struct EOF;

impl<'a> Parser<ParseState<'a>> for EOF {
  type Target = ();

  fn parse(&self, stream: &mut ParseState<'a>) -> Option<Self::Target> {
    match stream.next() {
      None => Some(()),
      Some(_) => None,
    }
  }
}
