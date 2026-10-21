use std::path::{Path, PathBuf};

use crate::error::{BlobstoreError, Result};

/// Returns the root directory of the repo
pub fn repo_root(path: &Path) -> Result<Option<PathBuf>> {
    for path in path.ancestors() {
        if path.join(".bs").is_dir() {
            return Ok(Some(path.to_path_buf()));
        }
    }

    Ok(None)
}

/// Returns `true` if the blobstore is initialised else will return `false`
pub fn is_blobstore_repo(path: &Path) -> Result<bool> {
    match repo_root(path)? {
        Some(_) => Ok(true),
        None => Ok(false),
    }
}

/// Use when an initialized repo root is required
pub fn required_repo_root(path: &Path) -> Result<PathBuf> {
    repo_root(path)?.ok_or(BlobstoreError::NotInRepository)
}
