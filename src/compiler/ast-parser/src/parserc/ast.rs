use vsp_ast::ast::AST;

pub fn parser() -> AST {
  AST::new("")
}

#[derive(Default)]
pub struct ASTParser;

impl ASTParser {
  pub fn parse_file(&mut self, content: &str) -> Result<(), ()> {
    Ok(())
  }
}
