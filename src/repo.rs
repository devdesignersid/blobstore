use std::{env, path::PathBuf};

use crate::error::{BlobstoreError, Result};

/// Returns the root directory of the repo
pub fn repo_root() -> Result<Option<PathBuf>> {
    let cwd = env::current_dir()?;

    for path in cwd.ancestors() {
        if path.join(".bs").exists() {
            return Ok(Some(path.to_path_buf()));
        }
    }

    Ok(None)
}

/// Returns `true` if the blobstore is initialised else will return `false`
pub fn is_blobstore_repo() -> Result<bool> {
    match repo_root()? {
        Some(_) => Ok(true),
        None => Ok(false),
    }
}

/// Use when an initialized repo root is required
pub fn required_repo_root() -> Result<PathBuf> {
    repo_root()?.ok_or(BlobstoreError::NotInRepository)
}
