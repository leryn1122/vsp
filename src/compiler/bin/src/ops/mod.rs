use clap::ArgMatches;

use crate::CommandLine;

pub(crate) mod clean;
pub(crate) mod compile;
pub(crate) mod completion;
pub(crate) mod new;
#[allow(dead_code)]
pub(crate) mod ps;
pub(crate) mod repl;
pub(crate) mod test;

pub type CliHandler = fn(&mut CommandLine, &ArgMatches) -> anyhow::Result<()>;
