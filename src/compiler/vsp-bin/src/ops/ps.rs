use anyhow::anyhow;
use clap::arg;

use crate::Config;

pub(crate) fn cli() -> clap::Command {
  clap::Command::new("ps")
    .about("Process status tool")
    .arg(arg!(-q --quiet "Enable quiet mode: Print PIDs only."))
    .arg(arg!(-v --verbose "Enable verbose mode: Print arguments passed to the executables."))
}

#[allow(unused_variables)]
pub(crate) fn execute(config: &mut Config, args: &clap::ArgMatches) -> anyhow::Result<()> {
  let res = vsp_mx::process::list_all_vm_processes();
  if let Err(e) = res {
    return Err(anyhow!(e));
  }
  let vm_procs = res.unwrap();
  for vm_proc in vm_procs {
    // TODO Use cli-table API instead of print iteratively.
    println!("{}\t{}", vm_proc.get_pid(), vm_proc.get_cmd());
  }
  Ok(())
}
