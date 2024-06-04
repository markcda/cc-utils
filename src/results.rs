//! Result types for `ErrorResponse` (`salvo`) and `CliError` (`reqwest`) errors.

#[cfg(feature = "salvo")]
use crate::errors::ErrorResponse;

#[cfg(feature = "salvo")]
pub type MResult<T> = Result<T, ErrorResponse>;

#[cfg(feature = "reqwest")]
#[cfg(any(target_arch = "wasm32", target_arch = "wasm64"))]
use crate::errors::CliError;

#[cfg(feature = "reqwest")]
#[cfg(any(target_arch = "wasm32", target_arch = "wasm64"))]
pub type CResult<T> = Result<T, CliError>;
