//! Implementation of utilities for working with MessagePack with requests in `salvo` and `reqwest`.

#[cfg(feature = "salvo")]
#[cfg(not(any(target_arch = "wasm32", target_arch = "wasm64")))]
use crate::prelude::*;

#[cfg(feature = "salvo")]
#[cfg(not(any(target_arch = "wasm32", target_arch = "wasm64")))]
use serde::Deserialize;

#[cfg(feature = "salvo")]
#[cfg(not(any(target_arch = "wasm32", target_arch = "wasm64")))]
use salvo::Request;

#[cfg(feature = "salvo")]
#[cfg(not(any(target_arch = "wasm32", target_arch = "wasm64")))]
#[allow(async_fn_in_trait)]
pub trait MsgPackParser {
  async fn parse_msgpack<'de, T: Deserialize<'de>>(&'de mut self) -> MResult<T>;
  async fn parse_msgpack_with_max_size<'de, T: Deserialize<'de>>(&'de mut self, max_size: usize) -> MResult<T>;
}

#[cfg(feature = "salvo")]
#[cfg(not(any(target_arch = "wasm32", target_arch = "wasm64")))]
impl MsgPackParser for Request {
  /// Parse MessagePack body as type `T` from request with default max size limit.
  #[inline]
  async fn parse_msgpack<'de, T: Deserialize<'de>>(&'de mut self) -> MResult<T> {
    self
      .parse_msgpack_with_max_size(salvo::http::request::global_secure_max_size())
      .await
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
        tracing::debug!("{:?}", payload);
        return rmp_serde::from_slice::<T>(payload).consider(Some(StatusCode::BAD_REQUEST), None::<String>, true);
      }
    }
    Err(ErrorResponse {
      status_code: Some(StatusCode::BAD_REQUEST),
      error_text: "Bad content type, must be `application/msgpack`.".into(),
      original_text: None,
      public_error: true,
    })
  }
}
