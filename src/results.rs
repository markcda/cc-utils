//! Result types for `ErrorResponse` (`salvo`) and `CliError` (`reqwest`) errors.

#[cfg(not(any(target_arch = "wasm32", target_arch = "wasm64")))]
use crate::errors::ErrorResponse;

#[cfg(not(any(target_arch = "wasm32", target_arch = "wasm64")))]
pub type MResult<T> = Result<T, ErrorResponse>;

#[cfg(any(target_arch = "wasm32", target_arch = "wasm64"))]
use crate::errors::CliError;

#[cfg(any(target_arch = "wasm32", target_arch = "wasm64"))]
pub type CResult<T> = Result<T, CliError>;
