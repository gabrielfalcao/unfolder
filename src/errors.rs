use std::fmt::Display;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Error {
    IOError(String),
    FlattenFileInputError(String),
    FlattenFileOutputError(String),
    UnflattenFileInputError(String),
    UnflattenFileOutputError(String),
    InvalidIndexError(String),
    MissingIndexError(String),
    UnreadableIndexError(String),
    CorruptedDataError(String),
    ChecksumMismatch(String),
}
impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}: {}",
            self.variant(),
            match self {
                Self::IOError(e) => e.to_string(),
                Self::FlattenFileInputError(e) => e.to_string(),
                Self::FlattenFileOutputError(e) => e.to_string(),
                Self::UnflattenFileInputError(e) => e.to_string(),
                Self::UnflattenFileOutputError(e) => e.to_string(),
                Self::InvalidIndexError(e) => e.to_string(),
                Self::MissingIndexError(e) => e.to_string(),
                Self::UnreadableIndexError(e) => e.to_string(),
                Self::CorruptedDataError(e) => e.to_string(),
                Self::ChecksumMismatch(e) => e.to_string(),
            }
        )
    }
}

impl Error {
    pub fn variant(&self) -> String {
        match self {
            Error::IOError(_) => "IOError",
            Self::FlattenFileInputError(_) => "FlattenFileInputError",
            Self::FlattenFileOutputError(_) => "FlattenFileOutputError",
            Self::UnflattenFileInputError(_) => "UnflattenFileInputError",
            Self::UnflattenFileOutputError(_) => "UnflattenFileOutputError",
            Self::InvalidIndexError(_) => "InvalidIndexError",
            Self::MissingIndexError(_) => "MissingIndexError",
            Self::UnreadableIndexError(_) => "UnreadableIndexError",
            Self::CorruptedDataError(_) => "CorruptedDataError",
            Self::ChecksumMismatch(_) => "ChecksumMismatch",
       }
        .to_string()
    }
}

impl std::error::Error for Error {}
impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Error::IOError(format!("{}", e))
    }
}
impl From<iocore::Error> for Error {
    fn from(e: iocore::Error) -> Self {
        Error::IOError(format!("{}", e))
    }
}
pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Clone)]
pub enum Exit {
    Success,
    Error(Error),
}
impl std::process::Termination for Exit {
    fn report(self) -> std::process::ExitCode {
        match &self {
            Exit::Success => std::process::ExitCode::from(0),
            Exit::Error(error) => {
                eprintln!("{}", error);
                std::process::ExitCode::from(1)
            },
        }
    }
}
impl<T> From<std::result::Result<T, Error>> for Exit {
    fn from(result: std::result::Result<T, Error>) -> Exit {
        match result {
            Ok(_) => Exit::Success,
            Err(e) => Exit::Error(e),
        }
    }
}
