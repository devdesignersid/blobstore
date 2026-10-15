use std::{
    env,
    io::{
        Error,
        ErrorKind::{self},
    },
};

pub fn repo_root() -> Result<Option<String>, Error> {
    let mut curr_path = env::current_dir()?;

    while curr_path.to_str().as_slice().len() > 0 {
        if curr_path.join(".bs").exists() {
            break;
        }

        let parent = match curr_path.parent() {
            Some(path) => path,
            None => break,
        };

        curr_path = parent.to_path_buf();
    }

    if curr_path.to_str().unwrap() == "/" {
        return Ok(None);
    }

    match curr_path.into_string() {
        Ok(str) => Ok(Some(str)),
        _ => return Err(Error::new(ErrorKind::InvalidData, "Invalid path")),
    }
}

pub fn check_repo_initialized() -> Result<bool, Error> {
    let is_repo_initialized = match repo_root() {
        Ok(Some(_)) => true,
        Ok(None) => false,
        Err(err) => return Err(Error::new(ErrorKind::Other, format!("{err}"))),
    };

    Ok(is_repo_initialized)
}
