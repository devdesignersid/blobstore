use std::{
    env, fs,
    io::{self, Error, ErrorKind},
};

use crate::repo::check_repo_initialized;

pub fn init() -> Result<(), io::Error> {
    if check_repo_initialized()? {
        return Err(Error::new(
            ErrorKind::AlreadyExists,
            "Repo already initialized",
        ));
    }

    let path = env::current_dir()?.join(".bs");

    fs::create_dir(&path)?;
    fs::create_dir(path.join("objects"))?;
    fs::create_dir(path.join("refs"))?;
    fs::write(path.join("HEAD"), "ref: refs/heads/master\n")?;
    println!("Initialized blobstore directory");
    Ok(())
}
