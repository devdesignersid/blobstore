use std::{env, fs, path::Path};

use crate::{
    error::{BlobstoreError, Result},
    repo::is_blobstore_repo,
};

/// Initialize blobstore repo in the current working directory
pub fn init() -> Result<()> {
    let cwd = env::current_dir()?;

    init_at(&cwd)?;
    println!("Initialized blobstore directory");
    Ok(())
}

pub fn init_at(root: &Path) -> Result<()> {
    if is_blobstore_repo(root)? {
        return Err(BlobstoreError::RepoAlreadyInitialised);
    }

    let path = root.join(".bs");

    fs::create_dir(&path)?;
    fs::create_dir(path.join("objects"))?;
    fs::create_dir(path.join("refs"))?;
    fs::write(path.join("HEAD"), "ref: refs/heads/master\n")?;
    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;
    use tempfile::tempdir;

    #[test]
    fn init_creates_repsository_structure() -> Result<()> {
        let temp = tempdir()?;

        init_at(temp.path())?;

        let repo = temp.path().join(".bs");

        assert!(repo.is_dir());
        assert!(repo.join("objects").is_dir());
        assert!(repo.join("refs").is_dir());

        let head = fs::read_to_string(repo.join("HEAD"))?;

        assert_eq!(head, "ref: refs/heads/master\n");
        Ok(())
    }

    #[test]
    fn init_rejects_existing_repository() -> Result<()> {
        let temp = tempdir()?;

        init_at(temp.path())?;

        let result = init_at(temp.path());

        assert!(matches!(
            result,
            Err(BlobstoreError::RepoAlreadyInitialised)
        ));

        Ok(())
    }

    #[test]
    fn init_rejects_nested_repository() -> Result<()> {
        let temp = tempdir()?;

        init_at(temp.path())?;

        let nested = temp.path().join("nested");
        fs::create_dir(&nested)?;

        let result = init_at(&nested);

        assert!(matches!(
            result,
            Err(BlobstoreError::RepoAlreadyInitialised)
        ));

        assert!(!nested.join(".bs").exists());

        Ok(())
    }

    #[test]
    fn init_fails_when_bs_path_is_a_file() -> Result<()> {
        let temp = tempdir()?;

        let bs_path = temp.path().join(".bs");
        fs::write(&bs_path, "existing file")?;

        let result = init_at(temp.path());

        assert!(matches!(result, Err(BlobstoreError::Io(_))));

        assert!(bs_path.is_file());

        Ok(())
    }
}
