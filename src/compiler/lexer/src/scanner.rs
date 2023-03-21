use std::fs::File;
use std::io::BufReader;

#[allow(dead_code)]
pub(crate) const EOF_CHAR: char = '\0';

#[allow(dead_code)]
pub struct Scanner {
  reader: BufReader<File>,
}

impl Scanner {
  pub fn from(file: File) -> Self {
    let reader = BufReader::new(file);
    Self { reader }
  }
}
