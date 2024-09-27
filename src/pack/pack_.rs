use std::{fs::read_dir, path::Path};
use flate2::{
  write::GzEncoder, 
  Compression
};
use crate::error::Result;

pub fn pack_bytes(directory: &Path) -> Result<Vec<u8>> {
  let encoder = GzEncoder::new(Vec::new(), Compression::default());
  let mut tar = tar::Builder::new(encoder);
  for entry in read_dir(directory)? {
    let path = entry?.path();
    if path.is_dir() {
      let dirname = file_name(&path);
      match dirname.as_str() {
        ".git" => continue,
        "node_modules" => continue,
        _ => tar.append_dir_all(dirname, path.clone())?
      }
    }
    if path.is_file() {
      tar.append_path_with_name(path.clone(), file_name(&path))?;
    }
  }
  tar.finish()?;

  Ok(tar.into_inner()?.finish()?)
}

fn file_name(path: &Path) -> String {
  path
    .file_name()
    .unwrap()
    .to_os_string()
    .into_string()
    .unwrap()
}

#[cfg(test)]
mod tests {
  use super::*;
  use std::path::PathBuf;

  #[test]
  fn test_pack() -> Result<()> {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("src").join("pack");
    let packed_bytes = pack_bytes(&path)?;

    let target = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("target").join("tests").join("pack.tar.gz");
    std::fs::create_dir_all(target.parent().unwrap())?;
    std::fs::write(&target, packed_bytes)?;

    assert!(target.exists());
    assert!(target.is_file());

    std::fs::remove_file(&target)?;

    Ok(())
  }
}