use std::io::Stdout;
use std::io::Write;
use std::path::Path;
use std::path::PathBuf;

use vsp_ast_parser::lex::DefaultLexer;
use vsp_error::VspError;
use vsp_error::VspResult;

#[repr(u8)]
pub enum DumpType {
  Token,
  Preprocessor,
  AST,
  LLVM,
}

impl From<u8> for DumpType {
  fn from(value: u8) -> Self {
    match value {
      0 => Self::Token,
      1 => Self::Preprocessor,
      2 => Self::AST,
      3 => Self::LLVM,
      _ => unreachable!("Unexpected value"),
    }
  }
}

pub struct Dumper {
  dump_type:     DumpType,
  path:          Option<PathBuf>,
  output_stream: Box<dyn Write>,
}

pub fn create_dumper(dump_type: DumpType) -> Dumper {
  Dumper {
    dump_type,
    path: None,
    output_stream: Box::new(std::io::stdout()),
  }
}

impl Dumper {
  pub fn from<P>(&mut self, path: P) -> &Dumper
  where
    P: AsRef<Path>,
  {
    self.path = Some(path.as_ref().to_path_buf());
    self
  }

  pub fn dump(&mut self) -> VspResult<()> {
    let content =
      std::fs::read_to_string(self.path.clone().unwrap()).map_err(|e| VspError::from(e))?;
    &self.dump_token(content.as_str());
    Ok(())
  }

  fn dump_token(&mut self, str: &str) {
    let mut lexer = DefaultLexer {};
    let result = lexer.tokenize(str).unwrap();
    result.to_vec().iter().for_each(|t| {
      let _ = std::io::stdout().write(format!("{}\n", t).as_bytes());
    })
  }
}
