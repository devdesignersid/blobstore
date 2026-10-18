use crate::error::{BlobstoreError::InvalidHash, Result};

pub fn validate_hash(hash: &str) -> Result<()> {
    if hash.len() != 40 || !hash.bytes().all(|byte| byte.is_ascii_hexdigit()) {
        return Err(InvalidHash);
    }
    Ok(())
}

pub fn next_null(bytes: &[u8], start: usize) -> Option<usize> {
    bytes
        .get(start..)?
        .iter()
        .position(|&byte| byte == 0)
        .map(|index| start + index)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accepts_valid_hash() {
        let hash = "0123456789abcdef0123456789abcdef01234567";
        assert!(validate_hash(hash).is_ok());
    }
}
