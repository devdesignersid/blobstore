use std::{
    fs,
    io::{self, Read, Write},
    path::Path,
};

use flate2::read::ZlibDecoder;

pub fn pretty_print(hash: &str) -> Result<(), io::Error> {
    if hash.len() < 3 || !hash.bytes().all(|byte| byte.is_ascii_hexdigit()) {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "invalid object ID",
        ));
    }

    let path = Path::new(".bs")
        .join("objects")
        .join(&hash[..2])
        .join(&hash[2..]);

    let mut decoded = Vec::new();
    ZlibDecoder::new(fs::File::open(path)?).read_to_end(&mut decoded)?;

    let separator = decoded
        .iter()
        .position(|&byte| byte == 0)
        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "missing object header"))?;

    let (header, contents_with_separator) = decoded.split_at(separator);
    let contents = &contents_with_separator[1..];

    let header = std::str::from_utf8(header)
        .map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "invalid object header"))?;

    let (kind, size) = header
        .split_once(" ")
        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "invalid object header"))?;

    let size: usize = size
        .parse()
        .map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "invalid object size"))?;

    if kind != "blob" || size != contents.len() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "invalid blob type or size",
        ));
    }

    io::stdout().lock().write_all(contents)?;
    Ok(())
}
