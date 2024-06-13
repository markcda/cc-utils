//! Implementation of utilities for working with responses in `salvo` and `reqwest`.

use crate::prelude::*;

#[cfg(feature = "salvo")]
#[cfg(not(any(target_arch = "wasm32", target_arch = "wasm64")))]
use salvo::http::HeaderValue;

#[cfg(feature = "salvo")]
#[cfg(not(any(target_arch = "wasm32", target_arch = "wasm64")))]
use salvo::hyper::header::CONTENT_TYPE;

#[cfg(feature = "salvo")]
#[cfg(not(any(target_arch = "wasm32", target_arch = "wasm64")))]
use salvo::oapi::{EndpointOutRegister, ToSchema};

#[cfg(feature = "salvo")]
#[cfg(not(any(target_arch = "wasm32", target_arch = "wasm64")))]
use salvo::{Request, Response, Depot};

#[cfg(feature = "salvo")]
#[cfg(not(any(target_arch = "wasm32", target_arch = "wasm64")))]
use salvo::Writer as ServerResponseWriter;

#[cfg(feature = "salvo")]
#[cfg(not(any(target_arch = "wasm32", target_arch = "wasm64")))]
use salvo::fs::NamedFile;

#[cfg(not(any(target_arch = "wasm32", target_arch = "wasm64")))]
use serde::Serialize;

#[cfg(feature = "reqwest")]
#[cfg(any(target_arch = "wasm32", target_arch = "wasm64"))]
use serde::de::DeserializeOwned;

/// Macro to define the function that called the response.
#[macro_export]
macro_rules! fn_name {
  () => {{
    fn f() {}
    fn type_name_of<T>(_: T) -> &'static str {
      std::any::type_name::<T>()
    }
    let name = type_name_of(f);
    
    // For `#[endpoint]` path can be shortened as follows:
    match name[..name.len() - 3].rsplit("::").nth(2) {
      Some(el) => el,
      None => &name[..name.len() - 3],
    }
  }};
}

/// Macro for automating `EndpointOutRegister` implementations (for simple types)
#[cfg(feature = "salvo")]
macro_rules! impl_oapi_endpoint_out {
  ($t:tt, $c:expr) => {
    impl EndpointOutRegister for $t {
      #[inline]
      fn register(components: &mut salvo::oapi::Components, operation: &mut salvo::oapi::Operation) {
        operation.responses.insert(
          "200",
          salvo::oapi::Response::new("Ok").add_content($c, String::to_schema(components)),
        );
      }
    }
  };
}

/// Macro for automating `EndpointOutRegister` implementations (for template types)
#[cfg(feature = "salvo")]
macro_rules! impl_oapi_endpoint_out_t {
  ($t:tt, $c:expr) => {
    impl<T> EndpointOutRegister for $t<T> {
      #[inline]
      fn register(components: &mut salvo::oapi::Components, operation: &mut salvo::oapi::Operation) {
        operation.responses.insert(
          "200",
          salvo::oapi::Response::new("Ok").add_content($c, String::to_schema(components)),
        );
      }
    }
  };
}

/// Sends 200 without data.
#[cfg(feature = "salvo")]
#[cfg(not(any(target_arch = "wasm32", target_arch = "wasm64")))]
pub struct OK(pub &'static str);

#[cfg(feature = "salvo")]
#[cfg(not(any(target_arch = "wasm32", target_arch = "wasm64")))]
impl_oapi_endpoint_out!(OK, "text/plain");

#[cfg(feature = "salvo")]
#[cfg(not(any(target_arch = "wasm32", target_arch = "wasm64")))]
#[macro_export]
macro_rules! ok { () => { Ok(OK(cc_utils::fn_name!())) }; }

#[cfg(feature = "salvo")]
#[cfg(not(any(target_arch = "wasm32", target_arch = "wasm64")))]
#[salvo::async_trait]
impl ServerResponseWriter for OK {
  async fn write(self, _req: &mut Request, _depot: &mut Depot, res: &mut Response) {
    res.status_code(StatusCode::OK);
    res.render("");
    log::debug!("[{}] => Received and sent result 200", self.0);
  }
}

/// Sends 200 and plain text.
#[cfg(feature = "salvo")]
#[cfg(not(any(target_arch = "wasm32", target_arch = "wasm64")))]
#[derive(Debug)]
pub struct Plain(pub String, pub &'static str);

#[cfg(feature = "salvo")]
#[cfg(not(any(target_arch = "wasm32", target_arch = "wasm64")))]
impl_oapi_endpoint_out!(Plain, "text/plain");

#[cfg(feature = "salvo")]
#[cfg(not(any(target_arch = "wasm32", target_arch = "wasm64")))]
#[macro_export]
macro_rules! plain { ($e:expr) => { Ok(Plain($e, cc_utils::fn_name!())) }; }

#[cfg(feature = "salvo")]
#[cfg(not(any(target_arch = "wasm32", target_arch = "wasm64")))]
#[salvo::async_trait]
impl ServerResponseWriter for Plain {
  async fn write(self, _req: &mut Request, _depot: &mut Depot, res: &mut Response) {
    res.status_code(StatusCode::OK);
    res.render(&self.0);
    log::debug!("[{}] => Received and sent result 200 with text: {}", self.1, self.0);
  }
}

/// Sends 200 and HTML.
#[cfg(feature = "salvo")]
#[cfg(not(any(target_arch = "wasm32", target_arch = "wasm64")))]
#[derive(Debug)]
pub struct Html(pub String, pub &'static str);

#[cfg(feature = "salvo")]
#[cfg(not(any(target_arch = "wasm32", target_arch = "wasm64")))]
impl_oapi_endpoint_out!(Html, "text/html");

#[cfg(feature = "salvo")]
#[cfg(not(any(target_arch = "wasm32", target_arch = "wasm64")))]
#[macro_export]
macro_rules! html { ($e:expr) => { Ok(Html($e, cc_utils::fn_name!())) }; }

#[cfg(feature = "salvo")]
#[cfg(not(any(target_arch = "wasm32", target_arch = "wasm64")))]
#[salvo::async_trait]
impl ServerResponseWriter for Html {
  async fn write(self, _req: &mut Request, _depot: &mut Depot, res: &mut Response) {
    res.status_code(StatusCode::OK);
    res.render(salvo::writing::Text::Html(&self.0));
    log::debug!("[{}] => Received and sent result 200 with HTML", self.1);
  }
}

/// Sends 200 and file.
#[cfg(feature = "salvo")]
#[cfg(not(any(target_arch = "wasm32", target_arch = "wasm64")))]
#[derive(Debug)]
pub struct File(pub String, pub String, pub &'static str);

#[cfg(feature = "salvo")]
#[cfg(not(any(target_arch = "wasm32", target_arch = "wasm64")))]
impl_oapi_endpoint_out!(File, "application/octet-stream");

/// File response.
///
/// Usage:
///
/// ```rust
/// use cc_utils::prelude::*;
/// use salvo::prelude::*;
///
/// pub async fn some_endpoint() -> MResult<File> {
///   file!("filepath", "Normal file name")
/// }
/// ```
#[cfg(feature = "salvo")]
#[cfg(not(any(target_arch = "wasm32", target_arch = "wasm64")))]
#[macro_export]
macro_rules! file { ($e1:expr, $e2:expr) => { Ok(File($e1, $e2, cc_utils::fn_name!())) }; }

#[cfg(feature = "salvo")]
#[cfg(not(any(target_arch = "wasm32", target_arch = "wasm64")))]
#[salvo::async_trait]
impl ServerResponseWriter for File {
  async fn write(self, req: &mut Request, _depot: &mut Depot, res: &mut Response) {
    res.status_code(StatusCode::OK);
    NamedFile::builder(&self.0).attached_name(&self.1).use_last_modified(true).send(req.headers(), res).await;
    log::debug!("[{}] => Received and sent result 200 with file {}", self.2, self.1);
  }
}

/// Sends 200 and JSON.
#[cfg(feature = "salvo")]
#[cfg(not(any(target_arch = "wasm32", target_arch = "wasm64")))]
#[derive(Debug)]
pub struct Json<T>(pub T, pub &'static str);

#[cfg(feature = "salvo")]
#[cfg(not(any(target_arch = "wasm32", target_arch = "wasm64")))]
impl_oapi_endpoint_out_t!(Json, "application/json");

#[cfg(feature = "salvo")]
#[cfg(not(any(target_arch = "wasm32", target_arch = "wasm64")))]
#[macro_export]
macro_rules! json { ($e:expr) => { Ok(Json($e, cc_utils::fn_name!())) }; }

#[cfg(feature = "salvo")]
#[cfg(not(any(target_arch = "wasm32", target_arch = "wasm64")))]
#[salvo::async_trait]
impl<T: Serialize + Send> ServerResponseWriter for Json<T> {
  async fn write(self, _req: &mut Request, _depot: &mut Depot, res: &mut Response) {
    res.status_code(StatusCode::OK);
    res.render(salvo::writing::Json(self.0));
    log::debug!("[{}] => Received and sent result 200 with JSON", self.1);
  }
}

/// Sends 200 and MsgPack.
#[cfg(feature = "salvo")]
#[cfg(not(any(target_arch = "wasm32", target_arch = "wasm64")))]
#[derive(Debug)]
pub struct MsgPack<T>(pub T, pub &'static str);

#[cfg(feature = "salvo")]
#[cfg(not(any(target_arch = "wasm32", target_arch = "wasm64")))]
impl_oapi_endpoint_out_t!(MsgPack, "application/msgpack");

#[cfg(feature = "salvo")]
#[cfg(not(any(target_arch = "wasm32", target_arch = "wasm64")))]
#[macro_export]
macro_rules! msgpack { ($e:expr) => { Ok(MsgPack($e, cc_utils::fn_name!())) }; }

#[cfg(feature = "salvo")]
#[cfg(not(any(target_arch = "wasm32", target_arch = "wasm64")))]
#[salvo::async_trait]
impl<T: Serialize + Send> ServerResponseWriter for MsgPack<T> {
  async fn write(self, req: &mut Request, depot: &mut Depot, res: &mut Response) {
    res.status_code(StatusCode::OK);
    match rmp_serde::to_vec(&self.0) {
      Ok(bytes) => {
        res.headers_mut().insert(CONTENT_TYPE, HeaderValue::from_static("application/msgpack; charset=utf-8"));
        log::debug!("[{}] => Sending bytes: {:?}", self.1, bytes);
        res.write_body(bytes).ok();
        log::debug!("[{}] => Received and sent result 200 with MsgPack", self.1);
      }
      Err(e) => {
        log::error!("[{}] => Failed to serialize data: {:?}", e, self.1);
        ErrorResponse::from("Failed to serialize data.").with_500().build().write(req, depot, res).await;
      }
    }
  }
}

#[cfg(feature = "reqwest")]
#[allow(async_fn_in_trait)]
#[cfg(any(target_arch = "wasm32", target_arch = "wasm64"))]
pub trait MsgPackResponse {
  async fn msgpack<T: DeserializeOwned>(self) -> CResult<T>;
}

#[cfg(feature = "reqwest")]
#[cfg(any(target_arch = "wasm32", target_arch = "wasm64"))]
impl MsgPackResponse for reqwest::Response {
  async fn msgpack<T: DeserializeOwned>(self) -> CResult<T> {
    let full = self.bytes().await?;
    rmp_serde::from_slice(&full).consider_cli(None)
  }
}
