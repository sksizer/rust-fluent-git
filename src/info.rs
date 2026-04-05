use crate::error::SetupError;
use crate::builder::version;
use crate::builder::version::Version;
use std::path::PathBuf;
use which::which;

pub fn available() -> bool {
    which("git").is_ok()
}

pub fn path() -> Result<PathBuf, SetupError> {
    match which("git") {
        Ok(path) => Ok(path),
        Err(_) => Err(SetupError::NotInstalled),
    }
}

pub struct GitInfo {
    pub path: PathBuf,
    pub version: Version,
}

pub fn get() -> Result<GitInfo, SetupError> {
    let path = path()?;
    let version = version::version()?;
    Ok(GitInfo { path, version })
}
