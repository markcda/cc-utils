//! Implementation of utilities for working with MessagePack with requests in `salvo` and `reqwest`.

use crate::prelude::*;

#[cfg(any(target_arch = "wasm32", target_arch = "wasm64"))]
use serde::Serialize;

#[cfg(not(any(target_arch = "wasm32", target_arch = "wasm64")))]
use serde::{Deserialize, Serialize};

#[cfg(feature = "salvo")]
use salvo::Request;

#[cfg(feature = "reqwest")]
use reqwest::RequestBuilder;

#[cfg(feature = "salvo")]
#[salvo::async_trait]
pub trait MsgPackParser {
  async fn parse_msgpack<'de, T: Deserialize<'de>>(&'de mut self) -> MResult<T>;
  async fn parse_msgpack_with_max_size<'de, T: Deserialize<'de>>(&'de mut self, max_size: usize) -> MResult<T>;
}

#[cfg(feature = "salvo")]
#[salvo::async_trait]
impl MsgPackParser for Request {
  /// Parse MessagePack body as type `T` from request with default max size limit.
  #[inline]
  async fn parse_msgpack<'de, T: Deserialize<'de>>(&'de mut self) -> MResult<T> {
    self.parse_msgpack_with_max_size(salvo::http::request::secure_max_size()).await
  }
  
  /// Parse MessagePack body as type `T` from request with max size limit.
  #[inline]
  async fn parse_msgpack_with_max_size<'de, T: Deserialize<'de>>(&'de mut self, max_size: usize) -> MResult<T> {
    let ctype = self.content_type();
    if let Some(ctype) = ctype {
      if ctype.subtype() == salvo::http::mime::MSGPACK {
        let payload = self.payload_with_max_size(max_size).await?;
        let payload = if payload.is_empty() {
          "null".as_bytes()
        } else {
          payload.as_ref()
        };
        log::debug!("{:?}", payload);
        return Ok(
          rmp_serde::from_slice::<T>(payload).consider(Some(StatusCode::BAD_REQUEST), None, true)?
        );
      }
    }
    Err(
      ErrorResponse {
        status_code: Some(StatusCode::BAD_REQUEST),
        error_text: "Bad content type, must be `application/msgpack`.".into(),
        original_text: None,
        public_error: true
      }
    )
  }
}

#[cfg(feature = "reqwest")]
#[cfg(not(any(target_arch = "wasm32", target_arch = "wasm64")))]
pub trait MsgPackBuilder {
  fn msgpack<T: Serialize + ?Sized>(self, msgpack: &T) -> MResult<RequestBuilder>;
}

#[cfg(feature = "reqwest")]
#[cfg(not(any(target_arch = "wasm32", target_arch = "wasm64")))]
impl MsgPackBuilder for RequestBuilder {
  fn msgpack<T: Serialize + ?Sized>(self, msgpack: &T) -> MResult<RequestBuilder> {
    let (cli, mut req) = self.build_split();
    let mut error = None;
    if let Ok(req) = req.as_mut() {
      match rmp_serde::to_vec(msgpack) {
        Ok(body) => {
          if !req.headers().contains_key(reqwest::header::CONTENT_TYPE) {
            req.headers_mut().insert(reqwest::header::CONTENT_TYPE, reqwest::header::HeaderValue::from_static("application/msgpack"));
          }
          *req.body_mut() = Some(body.into());
        },
        Err(err) => { error = Some(err); },
      }
    }
    if let Some(err) = error {
      Err(err.to_string().into())
    } else {
      Ok(RequestBuilder::from_parts(cli, req?))
    }
  }
}

#[cfg(feature = "reqwest")]
#[cfg(any(target_arch = "wasm32", target_arch = "wasm64"))]
pub trait MsgPackBuilder {
  fn msgpack<T: Serialize + ?Sized>(self, msgpack: &T) -> CResult<reqwest::Request>;
}

#[cfg(feature = "reqwest")]
#[cfg(any(target_arch = "wasm32", target_arch = "wasm64"))]
impl MsgPackBuilder for RequestBuilder {
  fn msgpack<T: Serialize + ?Sized>(self, msgpack: &T) -> CResult<reqwest::Request> {
    let mut req = self.build();
    let mut error = None;
    if let Ok(req) = req.as_mut() {
      match rmp_serde::to_vec(msgpack) {
        Ok(body) => {
          if !req.headers().contains_key(reqwest::header::CONTENT_TYPE) {
            req.headers_mut().insert(reqwest::header::CONTENT_TYPE, reqwest::header::HeaderValue::from_static("application/msgpack"));
          }
          *req.body_mut() = Some(body.into());
        },
        Err(err) => { error = Some(err); },
      }
    }
    if let Some(err) = error {
      Err(err.to_string().into())
    } else {
      Ok(req?)
    }
  }
}
