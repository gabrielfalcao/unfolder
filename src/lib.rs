pub(crate) mod errors;
#[doc(hidden)]
pub use errors::Exit;
#[doc(inline)]
pub use errors::{Error, Result};
#[doc(hidden)]
pub mod cli;
#[doc(hidden)]
pub use cli::{ArgsDispatcher, ParserDispatcher, SubcommandDispatcher};

pub(crate) mod file;

#[doc(inline)]
pub use file::{fold_file, unfold_file, Progress, Action};
