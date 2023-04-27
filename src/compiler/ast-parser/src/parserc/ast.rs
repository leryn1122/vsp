use vsp_ast::ast::AST;

pub fn parser() -> AST {
  AST::new("")
}

pub struct ASTParser;

impl ASTParser {
  pub fn new() -> Self {
    Self {}
  }

  pub fn parse_file(&mut self, content: &str) -> Result<(), ()> {
    Ok(())
  }
}

#[cfg(test)]
mod tests {}
