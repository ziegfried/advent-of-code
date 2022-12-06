use anyhow::{anyhow, Result};
use std::fs::metadata;
use std::path::Path;

pub fn exists<P: AsRef<Path>>(path: &P) -> bool {
    metadata(path).is_ok()
}

pub fn is_file_empty<P: AsRef<Path>>(path: &P) -> Result<bool> {
    let meta = metadata(path)?;
    Ok(meta.len() == 0)
}

pub fn file_name(path: &Path) -> Result<String> {
    if let Some(file_name) = path.file_name() {
        if let Some(name) = file_name.to_str() {
            return Ok(String::from(name));
        }
    }
    Err(anyhow!("Unable to extract file name from path"))
}
