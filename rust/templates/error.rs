//!
//! Provides this crate's [`Error`] and [`Result`] types.
//!

use thiserror::Error;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

///
/// The `Error` type for this crate.
///
#[derive(Debug, Error)]
pub enum PackageNameError {
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    #[error("OsString to String conversion error; bytes: {:?}", bytes)]
    OsString { bytes: Vec<u8> },
}

///
/// A `Result` type that specifically uses this crate's `Error`.
///
pub type Result<T> = std::result::Result<T, PackageNameError>;
