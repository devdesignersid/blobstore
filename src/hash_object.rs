use std::{
    fs,
    io::{self, Read, Write},
    path::Path,
};

use flate2::write::ZlibEncoder;
use hex::ToHex;
use sha1::Digest;

use crate::{
    error::{BlobstoreError, Result},
    repo::required_repo_root,
};

pub fn hash_object(file_name: &str) -> Result<()> {
    let root = required_repo_root()?;
    let path = Path::new(file_name);

    let mut contents: Vec<u8> = Vec::new();
    fs::File::open(path)?.read_to_end(&mut contents)?;
    let mut object = format!("blob {}\0", contents.len()).into_bytes();
    object.extend_from_slice(&contents);

    let hash: String = sha1::Sha1::digest(&object).encode_hex();
    let (object_directory, object_file_name) = hash.split_at(2);

    if root
        .join(".bs")
        .join("objects")
        .join(object_directory)
        .join(object_file_name)
        .exists()
    {
        return Err(BlobstoreError::Io(io::Error::new(
            io::ErrorKind::AlreadyExists,
            "object already hashed",
        )));
    }

    let object_write_path = root.join(".bs").join("objects").join(object_directory);
    fs::create_dir(&object_write_path)?;

    let mut encoder = ZlibEncoder::new(Vec::new(), flate2::Compression::default());
    encoder.write_all(&object)?;
    let compressed = encoder.finish()?;

    fs::write(object_write_path.join(object_file_name), compressed)?;
    Ok(())
}
