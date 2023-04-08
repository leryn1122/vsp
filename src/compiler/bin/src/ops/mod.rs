use clap::ArgMatches;

use crate::Config;

pub(crate) mod clean;
pub(crate) mod compile;
pub(crate) mod debug;
pub(crate) mod lint;
pub(crate) mod new;
pub(crate) mod ps;
pub(crate) mod repl;
pub(crate) mod stack;
pub(crate) mod tar;
pub(crate) mod test;

pub type CliHandler = fn(&mut Config, &ArgMatches) -> anyhow::Result<()>;
