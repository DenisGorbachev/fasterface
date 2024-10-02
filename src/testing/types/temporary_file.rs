use crate::Outcome;
use derive_getters::Getters;
use std::fs::{create_dir_all, File};
use std::io;
use std::path::{Path, PathBuf};
use tempfile::TempDir;

#[derive(Getters, Debug)]
pub struct TemporaryFile {
    root: TempDir,
    path_buf: PathBuf,
    file: File,
}

impl TemporaryFile {
    pub fn create(path: impl AsRef<Path>) -> Outcome<Self> {
        let root = TempDir::new()?;
        let path_buf = root.as_ref().join(path);
        let file = touchp(&path_buf)?;
        Ok(Self {
            root,
            path_buf,
            file,
        })
    }
}

// TODO: Move to a separate module
fn touchp(path: &Path) -> io::Result<File> {
    if let Some(parent) = path.parent() {
        create_dir_all(parent)?;
    }
    File::create(path)
}
