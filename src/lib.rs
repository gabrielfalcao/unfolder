pub(crate) mod errors;
#[doc(inline)]
pub use errors::{Error, Exit, Result};
#[doc(hidden)]
pub mod cli;
#[doc(hidden)]
pub use cli::{ArgsDispatcher, ParserDispatcher, SubcommandDispatcher};

pub mod file;

#[doc(inline)]
pub use file::{
    checksum, fold_file, read_bytes_and_checksum, read_unfold_index,
    unfold_file, validate_checksum, Action, Progress, MAX_FILE_SIZE,
};
