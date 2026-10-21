use std::{
    env, fs,
    io::{self, Read, Write, stdout},
};

use flate2::read::ZlibDecoder;

use crate::{
    error::{BlobstoreError, Result},
    repo::required_repo_root,
    utils::next_null,
};

pub fn ls_tree(hash: &str) -> Result<()> {
    let cwd = env::current_dir()?;
    let root = required_repo_root(&cwd)?;

    let directory = &hash[..2];
    let file_name = &hash[2..];

    let path = root
        .join(".bs")
        .join("objects")
        .join(directory)
        .join(file_name);

    let mut decoded: Vec<u8> = Vec::new();
    ZlibDecoder::new(fs::File::open(path)?).read_to_end(&mut decoded)?;

    let header_end = next_null(&decoded, 0).ok_or_else(|| {
        BlobstoreError::Io(io::Error::new(
            io::ErrorKind::InvalidData,
            "invalid tree object",
        ))
    })?;

    let header = &decoded[..header_end];
    let size = header
        .strip_prefix(b"tree ")
        .and_then(|bytes| std::str::from_utf8(bytes).ok())
        .and_then(|text| text.parse::<usize>().ok())
        .ok_or_else(|| {
            return BlobstoreError::Io(io::Error::new(
                io::ErrorKind::InvalidData,
                "invalid tree object",
            ));
        })?;

    let content = &decoded[header_end + 1..];

    if content.len() != size {
        return Err(BlobstoreError::Io(io::Error::new(
            io::ErrorKind::InvalidData,
            "invalid tree object",
        )));
    }

    let mut cursor = 0;

    while cursor < content.len() {
        let separator = next_null(content, cursor).ok_or_else(|| {
            return BlobstoreError::Io(io::Error::new(
                io::ErrorKind::InvalidData,
                "invalid tree object",
            ));
        })?;

        let entry = &content[cursor..separator];

        let space = entry.iter().position(|&byte| byte == b' ').ok_or_else(|| {
            return BlobstoreError::Io(io::Error::new(
                io::ErrorKind::InvalidData,
                "invalid tree object",
            ));
        })?;

        let name = entry
            .get(space + 1..)
            .filter(|name| !name.is_empty())
            .ok_or_else(|| {
                return BlobstoreError::Io(io::Error::new(
                    io::ErrorKind::InvalidData,
                    "invalid tree object",
                ));
            })?;

        let hash_start = separator + 1;
        let hash_end = hash_start + 20;

        stdout().lock().write_all(name)?;
        stdout().lock().write_all(b"\n")?;

        cursor = hash_end;
    }

    Ok(())
}
