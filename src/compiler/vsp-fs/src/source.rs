use std::fs;
use std::path::Path;
use std::str::FromStr;

use anyhow::anyhow;

/// File loader for source code file.
pub trait FileLoader {
  /// Determine whether the source code file exists.
  fn exists(&self, path: &Path) -> bool;
  /// Read the contents of source code file.
  fn read_file(&self, path: &Path) -> anyhow::Result<String>;
}

/// A source code file loader to read plain source code file using `std::fs`.
pub struct PlainFileLoader {}

impl FileLoader for PlainFileLoader {
  fn exists(&self, path: &Path) -> bool {
    path.exists()
  }

  fn read_file(&self, path: &Path) -> anyhow::Result<String> {
    match fs::read_to_string(path) {
      Ok(s) => Ok(s),
      Err(e) => Err(anyhow!(e)),
    }
  }
}

#[allow(unused)]
pub struct SourceMap {
  loader: Box<dyn FileLoader>,
  hash_kind: SourceFileHashAlgorithm,
}

/// Available algorithm of source file hash.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum SourceFileHashAlgorithm {
  MD5Sum,
  SHA1Sum,
  SHA256Sum,
  SHA512Sum,
  // SHASum,
  // SHA224Sum,
}

impl FromStr for SourceFileHashAlgorithm {
  type Err = ();

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    match s {
      "md5" => Ok(SourceFileHashAlgorithm::MD5Sum),
      "sha1" => Ok(SourceFileHashAlgorithm::SHA1Sum),
      "sha256" => Ok(SourceFileHashAlgorithm::SHA256Sum),
      "sha512" => Ok(SourceFileHashAlgorithm::SHA512Sum),
      // "sha" => Ok(SourceFileHashAlgorithm::SHASum),
      // "sha224" => Ok(SourceFileHashAlgorithm::SHA224Sum),
      _ => Err(()),
    }
  }
}

/// Source file hash code.
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
struct SourceFileHash {
  pub algorithm: SourceFileHashAlgorithm,
  value: [u8; 32],
}

impl SourceFileHash {
  pub fn new(algorithm: SourceFileHashAlgorithm, src: &str) -> SourceFileHash {
    let mut hash = SourceFileHash {
      algorithm,
      value: Default::default(),
    };
    let len = hash.hash_len();
    let value = &mut hash.value[..len];
    let data = src.as_bytes();
    match algorithm {
      SourceFileHashAlgorithm::MD5Sum => {
        use md5::{Digest, Md5};
        value.copy_from_slice(&Md5::digest(data));
      }
      SourceFileHashAlgorithm::SHA1Sum => {
        todo!()
      }
      SourceFileHashAlgorithm::SHA256Sum => {
        todo!()
      }
      SourceFileHashAlgorithm::SHA512Sum => {
        todo!()
      }
    };
    hash
  }

  fn hash_len(&self) -> usize {
    match self.algorithm {
      SourceFileHashAlgorithm::MD5Sum => 16,
      SourceFileHashAlgorithm::SHA1Sum => 16,
      SourceFileHashAlgorithm::SHA256Sum => 16,
      SourceFileHashAlgorithm::SHA512Sum => 32,
    }
  }

  pub fn matches(&self, src: &str) -> bool {
    Self::new(self.algorithm, src) == *self
  }
}

#[derive(Copy, Clone)]
pub struct SourceFile {}
