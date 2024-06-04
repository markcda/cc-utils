//! Типы результатов для ошибок `ErrorResponse` (`salvo`) и `CliError` (`reqwest`).

#[cfg(feature = "salvo")]
use crate::errors::ErrorResponse;

#[cfg(feature = "salvo")]
pub type MResult<T> = Result<T, ErrorResponse>;

#[cfg(feature = "reqwest")]
use crate::errors::CliError;

#[cfg(feature = "reqwest")]
pub type CResult<T> = Result<T, CliError>;
