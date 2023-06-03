use std::fs::File;
use std::path::Path;
use std::path::PathBuf;

use flate2::write::GzEncoder;
use flate2::Compression;
use tar::Builder as TarBuilder;
use zip::write::FileOptions;
use zip::CompressionMethod;
use zip::ZipWriter;

use crate::debug_println;

pub fn create_tar_archive_by_dir(out: &PathBuf, dir: &PathBuf) -> std::io::Result<()> {
  let file = File::create(out)?;
  let encoder = GzEncoder::new(file, Compression::default());
  let mut builder = TarBuilder::new(encoder);
  for entry in std::fs::read_dir(dir)? {
    let path = entry?.path();
    let name = strip_prefix(&path, dir);
    if path.is_dir() {
      builder.append_dir_all(name, path)?;
    } else {
      let mut file = File::open(&path)?;
      builder.append_file(&name, &mut file)?;
    }
  }
  Ok(())
}

pub fn create_zip_archive_by_dir(out: &PathBuf, dir: &PathBuf) -> std::io::Result<()> {
  let file = File::create(out)?;
  let mut zip = ZipWriter::new(file);
  for entry in std::fs::read_dir(dir)? {
    let path = entry?.path();
    let name = strip_prefix(&path, dir);
    if path.is_dir() {
      zip.add_directory(name, FileOptions::default())?;
    } else {
      let options = FileOptions::default().compression_method(CompressionMethod::Deflated);
      zip.start_file(name, options)?;
      let mut file = File::open(path)?;
      std::io::copy(&mut file, &mut zip)?;
    }
  }
  Ok(())
}

fn strip_prefix<P>(path: &PathBuf, dir: P) -> String
where
  P: AsRef<Path>,
{
  return path
    .strip_prefix(dir.as_ref().to_str().unwrap())
    .unwrap()
    .to_path_buf()
    .to_str()
    .unwrap()
    .to_string();
}
