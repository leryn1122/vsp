use std::io::Error;

fn main() -> Result<(), Error> {
  // let outdir = match env::var_os("OUT_DIR") {
  //   None => return Ok(()),
  //   Some(outdir) => outdir,
  // };
  //
  // let mut cmd = Command::new("vsp")
  //   .about("Tests completions")
  //   .arg(Arg::new("file").help("some input file"))
  //   .subcommand(
  //     Command::new("test")
  //       .about("tests things")
  //       .arg(Arg::new("case").long("case").help("the case to test")),
  //   );
  // let path = generate_to(Zsh, &mut cmd, env!("CARGO_BIN_NAME"), outdir)?;
  //
  // println!("cargo:warning=completion file is generated: {:?}", path);

  Ok(())
}
