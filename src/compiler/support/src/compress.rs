use std::fs::File;
use std::path::Path;

use flate2::write::GzEncoder;
use flate2::Compression;
use tar::Builder;
use zip::write::FileOptions;
use zip::CompressionMethod;
use zip::ZipWriter;

pub fn create_tar_archive_by_dir<P1, P2>(out: P1, dir: P2) -> std::io::Result<()>
where
  P1: AsRef<Path>,
  P2: AsRef<Path>,
{
  let file = File::create(out)?;
  let encoder = GzEncoder::new(file, Compression::default());
  let mut builder = Builder::new(encoder);
  for entry in std::fs::read_dir(dir.as_ref())? {
    let path = entry?.path();
    let name = strip_prefix(&path, dir.as_ref());
    if path.is_dir() {
      builder.append_dir_all(name, path)?;
    } else {
      let mut file = File::open(&path)?;
      builder.append_file(&name, &mut file)?;
    }
  }
  Ok(())
}

pub fn create_zip_archive_by_dir<P1, P2>(out: P1, dir: P2) -> std::io::Result<()>
where
  P1: AsRef<Path>,
  P2: AsRef<Path>,
{
  let file = File::create(out)?;
  let mut zip = ZipWriter::new(file);
  for entry in std::fs::read_dir(dir.as_ref())? {
    let path = entry?.path();
    let name = strip_prefix(&path, dir.as_ref());
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

fn strip_prefix<P1, P2>(path: P1, dir: P2) -> String
where
  P1: AsRef<Path>,
  P2: AsRef<Path>,
{
  path
    .as_ref()
    .strip_prefix(dir.as_ref().to_str().unwrap())
    .unwrap()
    .to_path_buf()
    .to_str()
    .unwrap()
    .to_string()
}
