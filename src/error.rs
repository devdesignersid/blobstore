use std::{error::Error, fmt, io};

pub const USAGE: &str = "\
Usage:
    bs init
    bs cat-file -p hash
    bs hash-object -w <file>
    bs ls-tree --name-only <hash>
";

#[derive(Debug)]
pub enum BlobstoreError {
    Io(io::Error),
    InvalidCommand(String),
    NotInRepository,
    InvalidHash,
    RepoAlreadyInitialised,
}
impl fmt::Display for BlobstoreError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Io(err) => write!(f, "I/O error: {err}"),
            Self::InvalidCommand(args) => {
                write!(f, "Invalid Command: {args}\n\n{USAGE}")
            }
            Self::NotInRepository => write!(f, "not inside a blobstore repository"),
            Self::InvalidHash => write!(f, "invalid hash"),
            Self::RepoAlreadyInitialised => write!(f, "repo already initialized"),
        }
    }
}

impl Error for BlobstoreError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Io(err) => Some(err),
            _ => None,
        }
    }
}

impl From<io::Error> for BlobstoreError {
    fn from(err: io::Error) -> Self {
        Self::Io(err)
    }
}

pub type Result<T> = std::result::Result<T, BlobstoreError>;
