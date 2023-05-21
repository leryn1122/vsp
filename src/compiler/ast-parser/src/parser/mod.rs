use vsp_error::VspResult;
use vsp_token::Token;

pub struct DefaultParser {
  tokens: Vec<Token>,
}

impl DefaultParser {
  pub fn parse(&self) -> VspResult<()> {
    for token in &self.tokens {}
    Ok(())
  }
}

#[cfg(test)]
mod tests {
  use combine::Parser;

  use super::*;
  use crate::parserc::token::tokenize;

  #[test]
  fn test_tokenize() {
    let result = tokenize().parse("public func main () -> int { print return 0 ; }").unwrap();
    let parser = DefaultParser { tokens: result.0 };
  }
}
