use std::{
    env, fs,
    io::{self, ErrorKind, Read, Write},
};

use flate2::read::ZlibDecoder;

use crate::{
    error::{BlobstoreError, Result},
    repo::required_repo_root,
    utils::{next_null, validate_hash},
};

pub fn pretty_print(hash: &str) -> Result<()> {
    validate_hash(hash)?;
    let directory = &hash[..2];
    let file_name = &hash[2..];

    let cwd = env::current_dir()?;
    let root = required_repo_root(&cwd)?;
    let path = root
        .join(".bs")
        .join("objects")
        .join(directory)
        .join(file_name);

    let mut decoded = Vec::new();
    ZlibDecoder::new(fs::File::open(path)?).read_to_end(&mut decoded)?;

    let separator = next_null(&decoded, 0).ok_or(io::Error::new(
        ErrorKind::InvalidData,
        "invalid object header",
    ))?;

    let (header, contents_with_separator) = decoded.split_at(separator);
    let contents = &contents_with_separator[1..];

    let header = std::str::from_utf8(header).map_err(|_| {
        BlobstoreError::Io(io::Error::new(
            io::ErrorKind::InvalidData,
            "invalid object header",
        ))
    })?;

    let (kind, size) = header.split_once(" ").ok_or_else(|| {
        BlobstoreError::Io(io::Error::new(
            io::ErrorKind::InvalidData,
            "invalid object header",
        ))
    })?;

    let size: usize = size
        .parse()
        .map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "invalid object size"))?;

    if kind != "blob" || size != contents.len() {
        return Err(BlobstoreError::Io(io::Error::new(
            io::ErrorKind::InvalidData,
            "invalid object kind or size",
        )));
    }

    io::stdout().lock().write_all(contents)?;
    Ok(())
}
