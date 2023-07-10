//! # Extensions for `clap` package
//! It includes extensions for `clap` package for the project.
use std::ffi::OsStr;
use std::ffi::OsString;
use std::str::FromStr;

use clap::Arg;
use clap::builder::TypedValueParser;
use clap::Command;
use clap::Error;
use target_lexicon::Triple;

/// Typed value parser for `target_lexicon::triple`.
///
/// Usage:
/// ```rust
/// use clap::arg;
/// use clap::Command;
/// use target_lexicon::Triple;
/// use vsp_support::clap_ext::TripleValueParser;
///
/// let command = Command::new("test").arg(
///       arg!(--target <triple> "Capture target triple").value_parser(TripleValueParser::default()),
///     );
/// let matches = command
///       .try_get_matches_from(vec![
///         "test",
///         "--target",
///         Triple::host().to_string().as_str(),
///       ])
///       .unwrap();
/// let triple = matches.get_one::<Triple>("target").unwrap();
/// ```
#[derive(Clone, Default)]
pub struct TripleValueParser;

impl TypedValueParser for TripleValueParser {
  type Value = Triple;

  fn parse_ref(
    &self,
    cmd: &Command,
    arg: Option<&Arg>,
    value: &OsStr,
  ) -> Result<Self::Value, Error> {
    self.parse(cmd, arg, value.to_owned())
  }

  fn parse(
    &self,
    cmd: &Command,
    _arg: Option<&Arg>,
    value: OsString,
  ) -> Result<Self::Value, Error> {
    value
      .into_string()
      .map_err(|_| Error::new(clap::error::ErrorKind::InvalidUtf8).with_cmd(cmd))
      .map(|s| Triple::from_str(s.as_str()))
      .unwrap()
      .map_err(|e| Error::raw(clap::error::ErrorKind::InvalidValue, e.to_string()).with_cmd(cmd))
  }
}

#[cfg(test)]
mod tests {
  use clap::arg;

  use super::*;

  #[test]
  fn test_triple_value_parser() {
    let command = Command::new("test").arg(
      arg!(--target <triple> "Capture target triple").value_parser(TripleValueParser::default()),
    );
    let matches = command
      .try_get_matches_from(vec![
        "test",
        "--target",
        Triple::host().to_string().as_str(),
      ])
      .unwrap();
    let triple = matches.get_one::<Triple>("target").unwrap();
    assert_eq!(triple, &Triple::host())
  }
}
