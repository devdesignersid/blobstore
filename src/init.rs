use std::{env, fs};

use crate::{
    error::{BlobstoreError, Result},
    repo::is_blobstore_repo,
};

pub fn init() -> Result<()> {
    if is_blobstore_repo()? {
        return Err(BlobstoreError::RepoAlreadyInitialised);
    }

    let path = env::current_dir()?.join(".bs");

    fs::create_dir(&path)?;
    fs::create_dir(path.join("objects"))?;
    fs::create_dir(path.join("refs"))?;
    fs::write(path.join("HEAD"), "ref: refs/heads/master\n")?;
    println!("Initialized blobstore directory");
    Ok(())
}
