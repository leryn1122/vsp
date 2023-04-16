use vsp_ast::ast::AST;
use vsp_parserc::combinator::conjunct::choice;
use vsp_parserc::parser::ParseState;
use vsp_parserc::parser::Parser;

pub fn parser() -> AST {
  AST::new("")
}

pub struct ASTParser;

impl ASTParser {
  pub fn new() -> Self {
    Self {}
  }

  pub fn parse_file(&mut self, content: &str) -> Result<AST, ()> {
    let mut state = ParseState::new(content);
    self.parse(&mut state).ok_or(())
  }
}

impl<'a> Parser<ParseState<'a>> for ASTParser {
  type Target = AST;

  fn parse(&self, state: &mut ParseState<'a>) -> Option<Self::Target> {
    // choice(vec![]).parse(state);
    None
  }
}

#[cfg(test)]
mod tests {}
