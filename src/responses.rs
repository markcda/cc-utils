//! Implementation of utilities for working with responses in `salvo` and `reqwest`.

use crate::prelude::*;

#[cfg(feature = "salvo")]
use salvo::http::HeaderValue;

#[cfg(feature = "salvo")]
use salvo::hyper::header::CONTENT_TYPE;

#[cfg(feature = "salvo")]
use salvo::oapi::{EndpointOutRegister, ToSchema};

#[cfg(feature = "salvo")]
use salvo::{Request, Response};

#[cfg(feature = "salvo")]
use salvo::Writer as ServerResponseWriter;

#[cfg(feature = "salvo")]
use salvo::fs::NamedFile;

use serde::{de::DeserializeOwned, Serialize};

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
pub struct OK(pub &'static str);

#[cfg(feature = "salvo")]
impl_oapi_endpoint_out!(OK, "text/plain");

#[cfg(feature = "salvo")]
#[macro_export]
macro_rules! ok { () => { Ok(OK(fn_name!())) }; }

#[cfg(feature = "salvo")]
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
#[derive(Debug)]
pub struct Plain(pub String, pub &'static str);

#[cfg(feature = "salvo")]
impl_oapi_endpoint_out!(Plain, "text/plain");

#[cfg(feature = "salvo")]
#[macro_export]
macro_rules! plain { ($e:expr) => { Ok(Plain($e, fn_name!())) }; }

#[cfg(feature = "salvo")]
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
#[derive(Debug)]
pub struct Html(pub String, pub &'static str);

#[cfg(feature = "salvo")]
impl_oapi_endpoint_out!(Html, "text/html");

#[cfg(feature = "salvo")]
#[macro_export]
macro_rules! html { ($e:expr) => { Ok(Html($e, fn_name!())) }; }

#[cfg(feature = "salvo")]
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
#[derive(Debug)]
pub struct File(pub String, pub String, pub &'static str);

#[cfg(feature = "salvo")]
impl_oapi_endpoint_out!(File, "application/octet-stream");

#[cfg(feature = "salvo")]
#[macro_export]
macro_rules! file { ($e:expr) => { Ok(File($e, fn_name!())) }; }

#[cfg(feature = "salvo")]
#[salvo::async_trait]
impl ServerResponseWriter for File {
  async fn write(self, req: &mut Request, _depot: &mut Depot, res: &mut Response) {
    res.status_code(StatusCode::OK);
    NamedFile::builder(&self.0).attached_name(&self.1).send(req.headers(), res).await;
    log::debug!("[{}] => Received and sent result 200 with file {}", self.2, self.1);
  }
}

/// Sends 200 and JSON.
#[cfg(feature = "salvo")]
#[derive(Debug)]
pub struct Json<T>(pub T, pub &'static str);

#[cfg(feature = "salvo")]
impl_oapi_endpoint_out_t!(Json, "application/json");

#[cfg(feature = "salvo")]
#[macro_export]
macro_rules! json { ($e:expr) => { Ok(Json($e, fn_name!())) }; }

#[cfg(feature = "salvo")]
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
#[derive(Debug)]
pub struct MsgPack<T>(pub T, pub &'static str);

#[cfg(feature = "salvo")]
impl_oapi_endpoint_out_t!(MsgPack, "application/msgpack");

#[cfg(feature = "salvo")]
#[macro_export]
macro_rules! msgpack { ($e:expr) => { Ok(MsgPack($e, fn_name!())) }; }

#[cfg(feature = "salvo")]
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
pub trait MsgPackResponse {
  async fn msgpack<T: DeserializeOwned>(self) -> CResult<T>;
}

#[cfg(feature = "reqwest")]
impl MsgPackResponse for reqwest::Response {
  async fn msgpack<T: DeserializeOwned>(self) -> CResult<T> {
    let full = self.bytes().await?;
    rmp_serde::from_slice(&full).consider_cli(None)
  }
}
