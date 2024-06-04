#![allow(unused_imports)]

#[cfg(feature = "salvo")]
pub use crate::brotli;

#[cfg(feature = "salvo")]
pub use crate::requests::MsgPackParser;

#[cfg(feature = "reqwest")]
pub use crate::requests::MsgPackBuilder;

#[cfg(feature = "salvo")]
pub use crate::results::MResult;

#[cfg(feature = "reqwest")]
pub use crate::results::CResult;

#[cfg(feature = "salvo")]
pub use crate::responses::{OK, Plain, Html, File, Json, MsgPack};

#[cfg(feature = "reqwest")]
pub use crate::responses::MsgPackResponse;

#[cfg(feature = "salvo")]
pub use crate::errors::{ErrorResponse, Consider};

#[cfg(feature = "reqwest")]
pub use crate::errors::{CliError, ConsiderCli};

#[cfg(feature = "salvo")]
pub use crate::{fn_name, ok, plain, html, file, json, msgpack};

#[cfg(feature = "salvo")]
pub use salvo::oapi::endpoint;

#[cfg(feature = "salvo")]
pub use salvo::http::StatusCode;

#[cfg(feature = "salvo")]
pub use salvo::Depot;
