use std::collections::BTreeMap;
use std::fmt::{Display, Formatter};
use std::iter::IntoIterator;

use iocore::{Path, Size};
use sha2::{Sha256, Digest};

use crate::{Error, Result};

const MAX_FILE_SIZE: u64 = u32::MAX as u64;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Action {
    Fold,
    Unfold,
}
impl Display for Action {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Fold => "Fold",
                Self::Unfold => "Unfold",
            }
        )
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Progress {
    Start(Action),
    Chunk {
        index: usize,
        count: usize,
        action: Action,
    },
    End(Action),
}
impl Display for Progress {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Start(action) => format!("{action} start"),
                Self::End(action) => format!("{action} end"),
                Self::Chunk {
                    index,
                    count,
                    action,
                } => format!("{action} chunk {index}/{count}"),
            }
        )
    }
}

pub(crate) fn checksum(bytes: &[u8]) -> Vec<u8> {
    let mut hasher = Sha256::new();
    hasher.update(bytes);
    hasher.finalize().to_vec()
}

pub(crate) fn read_bytes_and_checksum(
    input_path: &Path,
) -> Result<(Vec<u8>, Vec<u8>)> {
    let bytes = input_path.read_bytes()?;
    let slice = bytes.as_slice();
    Ok((bytes.clone(), checksum(slice)))
}
pub(crate) fn validate_checksum(bytes: &[u8], expected: &[u8]) -> Result<()> {
    let actual = checksum(bytes);
    if actual.as_slice() != expected {
        let expected = hex::encode(expected);
        let actual = hex::encode(actual);
        Err(Error::ChecksumMismatch(format!(
            "expected: {expected} actual: {actual}"
        )))
    } else {
        Ok(())
    }
}

pub fn unfold_file<C: FnMut(Progress)>(
    input_path: &Path,
    output_path: &Path,
    mut progress: C,
) -> Result<Path> {
    let input_path = input_path.canonicalize()?.relative_to_cwd();
    if !input_path.is_file() {
        return Err(Error::FlattenFileInputError(format!(
            "{input_path} is not a file"
        )));
    }
    if output_path.exists() {
        return Err(Error::FlattenFileOutputError(format!(
            "{output_path} already exists"
        )));
    }
    let size = input_path.file_size();
    if size.as_u64() > MAX_FILE_SIZE {
        let max_file_size = Size::from(MAX_FILE_SIZE);
        return Err(Error::FlattenFileInputError(format!(
            "{input_path} is too large {size} (max = {max_file_size})"
        )));
    }
    progress(Progress::Start(Action::Unfold));
    let (bytes, sha256) = read_bytes_and_checksum(&input_path)?;
    let mut index = BTreeMap::<String, String>::new();
    index.insert("sha256".to_string(), hex::encode(&sha256));
    let max_chunk_size = u16::MAX as usize;
    let mut paths = Vec::<Path>::new();
    let index_path = output_path.join("index");
    paths.push(index_path.clone());
    let chunk_count = if bytes.len() > max_chunk_size {
        (f64::from(bytes.len() as u32) / f64::from(u16::MAX)).ceil() as usize
    } else {
        1usize
    };
    for (idx, chunk) in bytes.chunks(max_chunk_size).enumerate() {
        let chunk_index = idx + 1;
        let index_hex = format!("{idx:032x}");
        let name = hex::encode(&checksum(&chunk));
        let chunk_path = output_path.join(&name);
        progress(Progress::Chunk {
            index: chunk_index,
            count: chunk_count,
            action: Action::Unfold,
        });
        chunk_path
            .write(&chunk)
            .map_err(|error| {
                Error::FlattenFileOutputError(format!(
                    "failed to write chunk {chunk_index}/{chunk_count} to {chunk_path}: {error}"
                ))
            })?;
        index.insert(index_hex, name);
    }
    index_path.write(
        serde_yaml::to_string(&index)
            .map_err(|error| {
                Error::FlattenFileOutputError(format!(
                    "failed to serialize index as yaml: {error}"
                ))
            })?
            .as_bytes(),
    )?;
    progress(Progress::End(Action::Unfold));
    Ok(output_path.clone())
}

pub fn fold_file<C: FnMut(Progress)>(
    input_path: &Path,
    output_path: &Path,
    mut progress: C,
) -> Result<Path> {
    let input_path = input_path.canonicalize()?.relative_to_cwd();
    if !input_path.is_dir() {
        return Err(Error::UnflattenFileInputError(format!(
            "{input_path} is not a directory"
        )));
    }
    if output_path.exists() {
        return Err(Error::UnflattenFileOutputError(format!(
            "{output_path} already exists"
        )));
    }
    progress(Progress::Start(Action::Fold));
    let (sha256, input_paths) = read_unfold_index(&input_path)?;
    let mut bytes = Vec::<u8>::new();
    let chunk_count = input_paths.len();
    for (idx, chunk_path) in input_paths.into_iter().enumerate() {
        let chunk_index = idx + 1;

        let filename = chunk_path.name();
        let sha256 = hex::decode(&filename).map_err(|error| {
            Error::CorruptedDataError(format!(
                "invalid hex ({filename}) in chunk path {chunk_path}: {error}"
            ))
        })?;
        let chunk_bytes = chunk_path.read_bytes()?;
        validate_checksum(&chunk_bytes, &sha256).map_err(|error| {
            Error::CorruptedDataError(format!("in path {chunk_path}: {error}"))
        })?;
        progress(Progress::Chunk {
            index: chunk_index,
            count: chunk_count,
            action: Action::Fold,
        });
        bytes.extend(&chunk_bytes);
    }
    validate_checksum(&bytes, &sha256).map_err(|error| {
        Error::CorruptedDataError(format!(
            "invalid checksum at {input_path}: {error}"
        ))
    })?;
    output_path.mkdir_parents()?.write(&bytes)?;
    progress(Progress::End(Action::Fold));
    Ok(output_path.clone())
}

pub(crate) fn read_unfold_index(input_path: &Path) -> Result<(Vec<u8>, Vec<Path>)> {
    let index_path = input_path
        .join("index")
        .canonicalize()?
        .relative_to_cwd();
    if !index_path.exists() {
        return Err(Error::MissingIndexError(format!(
            "'{index_path}' does not exist"
        )));
    }
    if !index_path.is_file() {
        return Err(Error::UnreadableIndexError(format!(
            "'{index_path}' is not a readable file"
        )));
    }
    let yaml = index_path.read()?;
    let mut index = serde_yaml::from_str::<BTreeMap<String, String>>(&yaml)
        .map_err(|error| {
            Error::UnreadableIndexError(format!(
                "invalid yaml in '{index_path}': {error}"
            ))
        })?;

    let sha256 = match index.remove("sha256") {
        Some(sha256) => hex::decode(sha256.as_str()).map_err(|error| {
            Error::InvalidIndexError(format!(
                "invalid hex in 'sha256' field of '{index_path}': {error}"
            ))
        })?,
        None =>
            return Err(Error::InvalidIndexError(format!(
                "missing 'sha256' field in '{index_path}'"
            ))),
    };
    if index.is_empty() {
        return Err(Error::InvalidIndexError(format!(
            "empty index in '{index_path}'"
        )));
    };

    let mut ordered_index = Vec::<(usize, Path)>::new();
    for (key, value) in index.iter() {
        let ord = usize::from_str_radix(key, 16).map_err(|error| {
            Error::InvalidIndexError(format!(
                "invalid hex in field '{key}' of '{index_path}': {error}"
            ))
        })?;
        let path = input_path.join(value);
        if !path.exists() {
            return Err(Error::InvalidIndexError(format!(
                "'{key}' points to missing file '{path}'"
            )));
        }
        if !path.is_file() {
            return Err(Error::InvalidIndexError(format!(
                "'{key}' points to unreadable file '{path}'"
            )));
        }
        ordered_index.push((ord, path));
    }
    ordered_index.sort_by(|a, b| a.0.cmp(&b.0));
    let mut input_paths = Vec::<Path>::new();

    for (exp, (idx, path)) in ordered_index.into_iter().enumerate() {
        if exp == idx {
            input_paths.push(path);
        } else {
            let key = format!("{idx:x}");
            return Err(Error::InvalidIndexError(format!(
                "mismatch index {exp} != {idx} in key '{key}' pointing at path '{path}'"
            )));
        }
    }
    Ok((sha256, input_paths))
}
