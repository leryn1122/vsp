pub mod ast;
pub mod token;

pub struct IntegerParser;

#[cfg(test)]
mod tests {
  use combine::many1;
  use combine::parser::char::alpha_num;
  use combine::parser::char::space;
  use combine::sep_by;
  use combine::Parser;

  use super::*;

  #[test]
  pub fn parse() -> Result<(), ()> {
    let mut vec = vec![];
    let word = many1(alpha_num());
    let mut parser = sep_by(word, space()).map(|mut words: Vec<String>| {
      println!("{:?}", <Vec<String> as AsRef<Vec<String>>>::as_ref(&words));
      vec.push(words.pop().unwrap())
    });
    let result = parser.parse(
      "\
      func main() -> int {\
        print(\"Hello world!!\");
        return 0;
      }",
    );
    Ok(())
  }
}
