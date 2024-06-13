//! Fast access to nice things.

#[cfg(feature = "salvo")]
#[cfg(not(any(target_arch = "wasm32", target_arch = "wasm64")))]
pub use crate::brotli;

#[cfg(feature = "salvo")]
pub use crate::requests::MsgPackParser;

#[cfg(feature = "reqwest")]
pub use crate::requests::MsgPackBuilder;

#[cfg(feature = "salvo")]
#[cfg(not(any(target_arch = "wasm32", target_arch = "wasm64")))]
pub use crate::results::MResult;

#[cfg(any(target_arch = "wasm32", target_arch = "wasm64"))]
pub use crate::results::CResult;

#[cfg(feature = "salvo")]
pub use crate::responses::{OK, Plain, Html, File, Json, MsgPack};

#[cfg(feature = "reqwest")]
#[cfg(any(target_arch = "wasm32", target_arch = "wasm64"))]
pub use crate::responses::MsgPackResponse;

#[cfg(not(any(target_arch = "wasm32", target_arch = "wasm64")))]
pub use crate::errors::{ErrorResponse, Consider};

#[cfg(any(target_arch = "wasm32", target_arch = "wasm64"))]
pub use crate::errors::{CliError, ConsiderCli};

#[cfg(feature = "salvo")]
#[cfg(not(any(target_arch = "wasm32", target_arch = "wasm64")))]
pub use crate::{ok, plain, html, file, json, msgpack};

#[cfg(feature = "salvo")]
#[cfg(not(any(target_arch = "wasm32", target_arch = "wasm64")))]
pub use salvo::oapi::endpoint;

#[cfg(feature = "salvo")]
#[cfg(not(any(target_arch = "wasm32", target_arch = "wasm64")))]
pub use salvo::http::StatusCode;