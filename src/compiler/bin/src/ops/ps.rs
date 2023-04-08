use anyhow::anyhow;
use clap::arg;
use clap::ArgMatches;
use clap::Command;
use vsp_mx::process::list_all_vm_processes;

use crate::Config;

#[allow(dead_code)]
pub(crate) fn cli() -> Command {
  Command::new("ps")
    .about("Process status tool")
    .arg(arg!(-q --quiet "Enable quiet mode: Print PIDs only."))
    .arg(arg!(-v --verbose "Enable verbose mode: Print arguments passed to the executables."))
}

#[allow(dead_code, unused_variables)]
pub(crate) fn execute(config: &mut Config, args: &ArgMatches) -> anyhow::Result<()> {
  let res = list_all_vm_processes();
  if let Err(e) = res {
    return Err(anyhow!(e));
  }
  let vm_processes = res.unwrap();
  for vm_process in vm_processes {
    // TODO Use cli-table API instead of print iteratively.
    println!("{}\t{}", vm_process.get_pid(), vm_process.get_cmd());
  }
  Ok(())
}
