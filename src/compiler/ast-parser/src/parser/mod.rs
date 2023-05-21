use vsp_token::Token;

pub struct DefaultParser {
  tokens: Vec<Token>,
}

#[cfg(test)]
mod tests {
  use combine::Parser;

  use super::*;
  use crate::parserc::token::tokenize;

  #[test]
  fn test_tokenize() {
    let result = tokenize()
      .parse(
        "public func main () -> int {\
      print return 0 ;\
      }",
      )
      .unwrap();
  }
}
