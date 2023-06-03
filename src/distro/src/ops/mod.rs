use std::path::PathBuf;
use std::path::MAIN_SEPARATOR_STR as PATH_SEPARATOR;

use cargo::core::compiler::CompileMode;
use cargo::core::Shell;
use cargo::core::Workspace;
use cargo::ops::CompileOptions;
use cargo::util::homedir;
use cargo::Config;
use lazy_static::lazy_static;
use target_lexicon::Triple;

use crate::resources;
use crate::resources_str;
use crate::support::shell::*;

lazy_static! {
  // `<root>` as root directory of the repository.
  static ref REPO_PATH: PathBuf = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
    .join("..")
    .join("..")
    .canonicalize()
    .unwrap();
  // `<root>/src/target`
  static ref TARGET_PATH: PathBuf = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
    .join("..")
    .join("target")
    .canonicalize()
    .unwrap();
  // `<root>/src/compiler`
  static ref PROJECT_COMPILER_PATH: PathBuf = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
    .join("..")
    .join("compiler")
    .canonicalize()
    .unwrap();
}

pub fn build() -> std::io::Result<()> {
  let cwd = PROJECT_COMPILER_PATH.clone();

  let vec = vec![cargo_build, pack_release];
  for f in vec.into_iter() {
    match f(&cwd) {
      Ok(_) => {}
      Err(e) => return Err(e),
    }
  }
  return Ok(());
}

pub fn cargo_build(cwd: &PathBuf) -> std::io::Result<()> {
  let config = new_cargo_config(&cwd).unwrap();
  let workspace = Workspace::new(
    &cwd.join(&["bin", "Cargo.toml"].join(PATH_SEPARATOR)).canonicalize().unwrap(),
    &config,
  )
  .unwrap();

  let compile_options = CompileOptions::new(&workspace.config(), CompileMode::Build).unwrap();
  let _ = cargo::ops::compile(&workspace, &compile_options).unwrap();
  Ok(())
}

pub fn pack_release(cwd: &PathBuf) -> std::io::Result<()> {
  let package_name = format!("vsp-{}", Triple::host().to_string());
  let target_dir = &cwd.join("..").join("target");

  mkdir_p(target_dir, package_name.clone()).expect("Failed to create package directory");
  let package_path = target_dir.join(&package_name);

  let dirs = resources_str!("file-tree.csv")
    .split('\n')
    .filter(|&l| !l.is_empty() && !l.starts_with('#'))
    .map(|s| s.to_string())
    .collect::<Vec<String>>();

  dirs.iter().for_each(|d| {
    mkdir(&package_path, d).expect(format!("Failed to create target directory: {}", &d).as_str());
  });

  cp(
    &package_path,
    PROJECT_COMPILER_PATH
      .clone()
      .join(&["target", "debug", "vsp"].join(PATH_SEPARATOR))
      .into_os_string(),
    &["bin", "vsp"].join(PATH_SEPARATOR),
  )
  .expect("Failed to copy binary: vsp");

  for f in vec!["conf/env"] {
    cp(&package_path, resources!("conf/env"), "conf")
      .expect(format!("Failed to copy conf: {}", f).as_str());
  }

  for f in vec!["LICENSE-Apache-2.0", "LICENSE-MIT", "README.md"] {
    cp(&package_path, REPO_PATH.clone().join(f), ".")
      .expect(format!("Failed to copy file: {}", f).as_str());
  }

  if cfg!(target_family = "windows") {
    zip(
      &REPO_PATH.clone().join("src"),
      PathBuf::from(&["target", &package_name, "lib", "stdlib.zip"].join(PATH_SEPARATOR)),
      "stdlib",
    )
    .expect("");
  } else {
    tar(
      &REPO_PATH.clone().join("src"),
      PathBuf::from(&["target", &package_name, "lib", "stdlib.tar.gz"].join(PATH_SEPARATOR)),
      "stdlib",
    )
    .expect("Failed to tar");
  }

  Ok(())
}

/// Create a cargo config struct at the given path.
pub fn new_cargo_config(path: &PathBuf) -> std::io::Result<Config> {
  let cwd = std::env::current_dir().unwrap();
  let config = Config::new(
    Shell::default(),
    path.to_path_buf(),
    homedir(&cwd.as_path()).unwrap(),
  );
  Ok(config)
}
