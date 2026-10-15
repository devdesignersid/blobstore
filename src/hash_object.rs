use std::{
    fs,
    io::{Error, ErrorKind, Read, Write},
    path::Path,
};

use flate2::write::ZlibEncoder;
use hex::ToHex;
use sha1::Digest;

use crate::repo::repo_root;

pub fn hash_object(file_name: &str) -> Result<(), Error> {
    let repo_root_path = match repo_root() {
        Ok(Some(path)) => path,
        Ok(None) => {
            return Err(Error::new(
                ErrorKind::NotFound,
                "Not a blobstore repo! Initialize bobstore using `bs init` command",
            ));
        }
        Err(err) => return Err(Error::new(ErrorKind::Other, format!("{err}"))),
    };

    let path = Path::new(file_name);
    let mut contents: Vec<u8> = Vec::new();
    fs::File::open(path)?.read_to_end(&mut contents)?;
    let mut object = format!("blob {}\0", contents.len()).into_bytes();
    object.extend_from_slice(&contents);
    let hash: String = sha1::Sha1::digest(&object).encode_hex();
    let (object_directory, object_file_name) = hash.split_at(2);

    if Path::new(repo_root_path.as_str())
        .join(".bs")
        .join("objects")
        .join(object_directory)
        .join(object_file_name)
        .exists()
    {
        return Err(Error::new(ErrorKind::AlreadyExists, "File already exists"));
    }

    let object_write_path = Path::new(repo_root_path.as_str())
        .join(".bs")
        .join("objects")
        .join(object_directory);
    fs::create_dir(&object_write_path)?;

    let mut encoder = ZlibEncoder::new(Vec::new(), flate2::Compression::default());
    encoder.write_all(&object)?;
    let compressed = encoder.finish()?;

    fs::write(object_write_path.join(object_file_name), compressed)?;
    Ok(())
}
