pub(crate) mod errors;
pub use errors::{Error, Exit, Result};
pub mod cli;
pub use cli::{ArgsDispatcher, ParserDispatcher, SubcommandDispatcher};

pub mod file;
pub use file::{unfold_file, read_bytes_and_checksum, fold_file};
