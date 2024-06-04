//! Result types for `ErrorResponse` (`salvo`) and `CliError` (`reqwest`) errors.

#[cfg(feature = "salvo")]
use crate::errors::ErrorResponse;

#[cfg(feature = "salvo")]
pub type MResult<T> = Result<T, ErrorResponse>;

#[cfg(feature = "reqwest")]
use crate::errors::CliError;

#[cfg(feature = "reqwest")]
pub type CResult<T> = Result<T, CliError>;
