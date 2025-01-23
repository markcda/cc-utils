//! Implementation of optional private errors for `salvo` and client errors for `reqwest`.

#[cfg(feature = "salvo")]
#[cfg(not(any(target_arch = "wasm32", target_arch = "wasm64")))]
use std::any::Any;

#[cfg(feature = "salvo")]
#[cfg(not(any(target_arch = "wasm32", target_arch = "wasm64")))]
use salvo::http::StatusCode;

#[cfg(feature = "salvo")]
#[cfg(not(any(target_arch = "wasm32", target_arch = "wasm64")))]
use salvo::oapi::{EndpointOutRegister, ToSchema};

#[cfg(feature = "salvo")]
#[cfg(not(any(target_arch = "wasm32", target_arch = "wasm64")))]
use salvo::{Depot, Request, Response};

#[cfg(feature = "salvo")]
#[cfg(not(any(target_arch = "wasm32", target_arch = "wasm64")))]
use salvo::Writer as ServerResponseWriter;

pub type BoxDynError = Box<dyn std::error::Error + 'static + Send + Sync>;

/// Data structure responsible for server errors.
#[cfg(not(any(target_arch = "wasm32", target_arch = "wasm64")))]
#[derive(Debug)]
pub struct ErrorResponse {
  #[cfg(feature = "salvo")]
  #[cfg(not(any(target_arch = "wasm32", target_arch = "wasm64")))]
  pub status_code: Option<StatusCode>,
  pub error_text: String,
  pub original_text: Option<String>,
  pub public_error: bool,
}

#[cfg(not(any(target_arch = "wasm32", target_arch = "wasm64")))]
impl std::fmt::Display for ErrorResponse {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.write_str(&format!(
      r#"Error found! Status code to return - {}, original error text - "{}", public error text - "{}""#,
      self.status_code.unwrap_or(StatusCode::INTERNAL_SERVER_ERROR).as_str(),
      self.original_text.as_ref().unwrap_or(&"".to_string()),
      self.error_text.as_str(),
    ))
  }
}

#[cfg(not(any(target_arch = "wasm32", target_arch = "wasm64")))]
impl std::error::Error for ErrorResponse {}

/// Data structure responsible for client errors.
#[cfg(any(target_arch = "wasm32", target_arch = "wasm64"))]
#[derive(Debug, Clone)]
pub struct CliError {
  pub message: String,
}

#[cfg(any(target_arch = "wasm32", target_arch = "wasm64"))]
impl std::fmt::Display for CliError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.write_str(self.message.as_str())
  }
}

#[cfg(feature = "salvo")]
#[cfg(not(any(target_arch = "wasm32", target_arch = "wasm64")))]
#[salvo::async_trait]
impl ServerResponseWriter for ErrorResponse {
  /// Method for sending an error message to the client.
  async fn write(self, _req: &mut Request, _depot: &mut Depot, res: &mut Response) {
    res.status_code(self.status_code.unwrap_or(StatusCode::INTERNAL_SERVER_ERROR));
    if !self.public_error {
      let public_error_desc = match self.status_code {
        Some(StatusCode::BAD_REQUEST) => "Bad request.",
        Some(StatusCode::UNAUTHORIZED) => "Unauthorized request.",
        Some(StatusCode::FORBIDDEN) => "Access denied.",
        Some(StatusCode::NOT_FOUND) => "Page or method not found.",
        Some(StatusCode::METHOD_NOT_ALLOWED) => "Method not allowed.",
        Some(StatusCode::LOCKED) => "Your actions is locked.",
        Some(StatusCode::INTERNAL_SERVER_ERROR) => "Internal server error. Contact the administrator.",
        _ => "Specific error. Check with the administrator for details.",
      };
      tracing::error!(
        "Error with code {:?}: \"{}\", client will get: \"{}\"",
        self.status_code,
        self.error_text,
        public_error_desc
      );
      if self.original_text.is_some() {
        tracing::error!("The original error text: {:?}", self.original_text.unwrap());
      }
      res.render(public_error_desc);
    } else {
      tracing::error!("Error with code {:?}: \"{}\"", self.status_code, self.error_text);
      if self.original_text.is_some() {
        tracing::error!("The original error text: {:?}", self.original_text.unwrap());
      }
      res.render(&self.error_text);
    }
  }
}

#[cfg(feature = "salvo")]
#[cfg(not(any(target_arch = "wasm32", target_arch = "wasm64")))]
impl EndpointOutRegister for ErrorResponse {
  /// Registers error types for OpenAPI.
  fn register(components: &mut salvo::oapi::Components, operation: &mut salvo::oapi::Operation) {
    operation.responses.insert(
      "400",
      salvo::oapi::Response::new("Bad request").add_content("text/plain", String::to_schema(components)),
    );
    operation.responses.insert(
      "401",
      salvo::oapi::Response::new("Unauthorized").add_content("text/plain", String::to_schema(components)),
    );
    operation.responses.insert(
      "403",
      salvo::oapi::Response::new("Forbidden").add_content("text/plain", String::to_schema(components)),
    );
    operation.responses.insert(
      "404",
      salvo::oapi::Response::new("Not found").add_content("text/plain", String::to_schema(components)),
    );
    operation.responses.insert(
      "405",
      salvo::oapi::Response::new("Method not allowed").add_content("text/plain", String::to_schema(components)),
    );
    operation.responses.insert(
      "423",
      salvo::oapi::Response::new("Locked").add_content("text/plain", String::to_schema(components)),
    );
    operation.responses.insert(
      "500",
      salvo::oapi::Response::new("Internal server error").add_content("text/plain", String::to_schema(components)),
    );
  }
}

#[cfg(feature = "salvo")]
#[cfg(not(any(target_arch = "wasm32", target_arch = "wasm64")))]
#[allow(dead_code)]
impl ErrorResponse {
  /// Private error BAD REQUEST (400).
  pub fn with_400(&mut self) -> &mut Self {
    self.status_code = Some(StatusCode::BAD_REQUEST);
    self.public_error = false;
    self
  }

  /// Public error BAD REQUEST (400).
  pub fn with_400_pub(&mut self) -> &mut Self {
    self.status_code = Some(StatusCode::BAD_REQUEST);
    self.public_error = true;
    self
  }

  /// Private error UNAUTHORIZED (401).
  pub fn with_401(&mut self) -> &mut Self {
    self.status_code = Some(StatusCode::UNAUTHORIZED);
    self.public_error = false;
    self
  }

  /// Public error UNAUTHORIZED (401).
  pub fn with_401_pub(&mut self) -> &mut Self {
    self.status_code = Some(StatusCode::UNAUTHORIZED);
    self.public_error = true;
    self
  }

  /// Private error FORBIDDEN (403).
  pub fn with_403(&mut self) -> &mut Self {
    self.status_code = Some(StatusCode::FORBIDDEN);
    self.public_error = false;
    self
  }

  /// Public error FORBIDDEN (403).
  pub fn with_403_pub(&mut self) -> &mut Self {
    self.status_code = Some(StatusCode::FORBIDDEN);
    self.public_error = true;
    self
  }

  /// Private error NOT FOUND (404).
  pub fn with_404(&mut self) -> &mut Self {
    self.status_code = Some(StatusCode::NOT_FOUND);
    self.public_error = false;
    self
  }

  /// Public error NOT FOUND (404).
  pub fn with_404_pub(&mut self) -> &mut Self {
    self.status_code = Some(StatusCode::NOT_FOUND);
    self.public_error = true;
    self
  }

  /// Private error METHOD NOT ALLOWED (405).
  pub fn with_405(&mut self) -> &mut Self {
    self.status_code = Some(StatusCode::METHOD_NOT_ALLOWED);
    self.public_error = false;
    self
  }

  /// Public error METHOD NOT ALLOWED (405).
  pub fn with_405_pub(&mut self) -> &mut Self {
    self.status_code = Some(StatusCode::METHOD_NOT_ALLOWED);
    self.public_error = true;
    self
  }

  /// Private error LOCKED (423).
  pub fn with_423(&mut self) -> &mut Self {
    self.status_code = Some(StatusCode::LOCKED);
    self.public_error = false;
    self
  }

  /// Public error LOCKED (423).
  pub fn with_423_pub(&mut self) -> &mut Self {
    self.status_code = Some(StatusCode::LOCKED);
    self.public_error = true;
    self
  }

  /// Private error INTERNAL SERVER ERROR (500).
  pub fn with_500(&mut self) -> &mut Self {
    self.status_code = Some(StatusCode::INTERNAL_SERVER_ERROR);
    self.public_error = false;
    self
  }

  /// Public error INTERNAL SERVER ERROR (500).
  pub fn with_500_pub(&mut self) -> &mut Self {
    self.status_code = Some(StatusCode::INTERNAL_SERVER_ERROR);
    self.public_error = true;
    self
  }

  /// Changes error message text.
  pub fn with_text(&mut self, text: impl Into<String>) -> &mut Self {
    if self.original_text.is_none() {
      self.original_text = Some(self.error_text.to_owned());
    }
    self.error_text = text.into();
    self
  }

  /// Builds the response.
  pub fn build(&mut self) -> Self {
    Self {
      status_code: self.status_code,
      error_text: self.error_text.to_owned(),
      original_text: self.original_text.clone(),
      public_error: self.public_error,
    }
  }
}

/// A trait that allows you to transform any error into an `ErrorResponse` by assigning additional parameters.
#[cfg(feature = "salvo")]
#[cfg(not(any(target_arch = "wasm32", target_arch = "wasm64")))]
pub trait Consider<T> {
  fn consider(
    self,
    status_code: Option<StatusCode>,
    error_text_replacement: Option<impl Into<String>>,
    public: bool,
  ) -> Result<T, ErrorResponse>;
}

#[cfg(any(target_arch = "wasm32", target_arch = "wasm64"))]
pub trait ConsiderCli<T> {
  fn consider_cli(self, error_text_replacement: Option<String>) -> Result<T, CliError>;
}

#[cfg(feature = "salvo")]
#[cfg(not(any(target_arch = "wasm32", target_arch = "wasm64")))]
impl<T> Consider<T> for Result<T, ErrorResponse> {
  /// Changes the parameters of a possible error to the specified ones.
  fn consider(
    self,
    status_code: Option<StatusCode>,
    error_text_replacement: Option<impl Into<String>>,
    public: bool,
  ) -> Result<T, ErrorResponse> {
    self.map_err(|e| {
      let mut new_error = ErrorResponse {
        status_code,
        error_text: e.error_text,
        original_text: e.original_text,
        public_error: public,
      };
      if error_text_replacement.is_some() {
        new_error.original_text = Some(new_error.error_text.to_owned());
        new_error.error_text = error_text_replacement.unwrap().into();
      }
      new_error
    })
  }
}

#[cfg(any(target_arch = "wasm32", target_arch = "wasm64"))]
impl<T> ConsiderCli<T> for Result<T, CliError> {
  /// Changes the parameters of a possible error to the specified ones.
  fn consider_cli(self, error_text_replacement: Option<String>) -> Result<T, CliError> {
    self.map_err(|e| {
      let mut new_error = CliError { message: e.message };
      if error_text_replacement.is_some() {
        new_error.message = error_text_replacement.unwrap();
      }
      new_error
    })
  }
}

#[cfg(feature = "salvo")]
#[cfg(not(any(target_arch = "wasm32", target_arch = "wasm64")))]
impl<T> Consider<T> for Result<T, String> {
  /// Changes the parameters of a possible error to the specified ones.
  fn consider(
    self,
    status_code: Option<StatusCode>,
    error_text_replacement: Option<impl Into<String>>,
    public: bool,
  ) -> Result<T, ErrorResponse> {
    self.map_err(|e| {
      let mut new_error = ErrorResponse {
        status_code,
        error_text: e,
        original_text: None,
        public_error: public,
      };
      if error_text_replacement.is_some() {
        new_error.original_text = Some(new_error.error_text.to_owned());
        new_error.error_text = error_text_replacement.unwrap().into();
      }
      new_error
    })
  }
}

#[cfg(not(any(target_arch = "wasm32", target_arch = "wasm64")))]
impl<T> Consider<T> for anyhow::Result<T> {
  /// Changes the parameters of a possible error to the specified ones.
  fn consider(
    self,
    status_code: Option<StatusCode>,
    error_text_replacement: Option<impl Into<String>>,
    public: bool,
  ) -> Result<T, ErrorResponse> {
    self.map_err(|e| {
      let mut new_error = ErrorResponse {
        status_code,
        error_text: e.to_string(),
        original_text: None,
        public_error: public,
      };
      if error_text_replacement.is_some() {
        new_error.original_text = Some(new_error.error_text.to_owned());
        new_error.error_text = error_text_replacement.unwrap().into();
      }
      new_error
    })
  }
}

#[cfg(any(target_arch = "wasm32", target_arch = "wasm64"))]
impl<T> ConsiderCli<T> for Result<T, String> {
  /// Changes the parameters of a possible error to the specified ones.
  fn consider_cli(self, error_text_replacement: Option<String>) -> Result<T, CliError> {
    self.map_err(|e| {
      let mut new_error = CliError { message: e };
      if error_text_replacement.is_some() {
        new_error.message = error_text_replacement.unwrap();
      }
      new_error
    })
  }
}

#[cfg(feature = "salvo")]
#[cfg(not(any(target_arch = "wasm32", target_arch = "wasm64")))]
impl<T> Consider<T> for Result<T, &str> {
  /// Changes the parameters of a possible error to the specified ones.
  fn consider(
    self,
    status_code: Option<StatusCode>,
    error_text_replacement: Option<impl Into<String>>,
    public: bool,
  ) -> Result<T, ErrorResponse> {
    self.map_err(|e| {
      let mut new_error = ErrorResponse {
        status_code,
        error_text: e.to_owned(),
        original_text: None,
        public_error: public,
      };
      if error_text_replacement.is_some() {
        new_error.original_text = Some(new_error.error_text.to_owned());
        new_error.error_text = error_text_replacement.unwrap().into();
      }
      new_error
    })
  }
}

#[cfg(any(target_arch = "wasm32", target_arch = "wasm64"))]
impl<T> ConsiderCli<T> for Result<T, &str> {
  /// Changes the parameters of a possible error to the specified ones.
  fn consider_cli(self, error_text_replacement: Option<String>) -> Result<T, CliError> {
    self.map_err(|e| {
      let mut new_error = CliError { message: e.to_owned() };
      if error_text_replacement.is_some() {
        new_error.message = error_text_replacement.unwrap();
      }
      new_error
    })
  }
}

#[cfg(feature = "salvo")]
#[cfg(not(any(target_arch = "wasm32", target_arch = "wasm64")))]
impl From<String> for ErrorResponse {
  /// Creates a new error from a string.
  fn from(value: String) -> Self {
    Self {
      #[cfg(not(any(target_arch = "wasm32", target_arch = "wasm64")))]
      status_code: None,
      error_text: value,
      original_text: None,
      public_error: false,
    }
  }
}

#[cfg(any(target_arch = "wasm32", target_arch = "wasm64"))]
impl From<String> for CliError {
  /// Creates a new error from a string.
  fn from(value: String) -> Self {
    Self { message: value }
  }
}

#[cfg(feature = "salvo")]
#[cfg(not(any(target_arch = "wasm32", target_arch = "wasm64")))]
impl From<&str> for ErrorResponse {
  /// Creates a new error from a string.
  fn from(value: &str) -> Self {
    Self {
      #[cfg(not(any(target_arch = "wasm32", target_arch = "wasm64")))]
      status_code: None,
      error_text: value.to_owned(),
      original_text: None,
      public_error: false,
    }
  }
}

#[cfg(any(target_arch = "wasm32", target_arch = "wasm64"))]
impl From<&str> for CliError {
  /// Creates a new error from a string.
  fn from(value: &str) -> Self {
    Self {
      message: value.to_owned(),
    }
  }
}

/// Macro to simplify `Consider` trait implementation.
#[cfg(not(any(target_arch = "wasm32", target_arch = "wasm64")))]
macro_rules! impl_consider {
  ($e:ty) => {
    #[cfg(feature = "salvo")]
    #[cfg(not(any(target_arch = "wasm32", target_arch = "wasm64")))]
    impl<T> Consider<T> for Result<T, $e> {
      /// Изменяет параметры возможной ошибки на указанные.
      fn consider(
        self,
        status_code: Option<StatusCode>,
        error_text_replacement: Option<impl Into<String>>,
        public: bool,
      ) -> Result<T, ErrorResponse> {
        self.map_err(|e| {
          let mut new_error = ErrorResponse {
            status_code,
            error_text: e.to_string(),
            original_text: None,
            public_error: public,
          };
          if error_text_replacement.is_some() {
            new_error.original_text = Some(new_error.error_text.to_owned());
            new_error.error_text = error_text_replacement.unwrap().into();
          }
          new_error
        })
      }
    }

    #[cfg(feature = "salvo")]
    #[cfg(not(any(target_arch = "wasm32", target_arch = "wasm64")))]
    impl From<$e> for ErrorResponse {
      /// Создаёт `ErrorResponse` из данной ошибки.
      fn from(value: $e) -> Self {
        value.to_string().into()
      }
    }
  };
}

/// Macro to simplify `ConsiderCli` trait implementation.
#[cfg(any(target_arch = "wasm32", target_arch = "wasm64"))]
macro_rules! impl_consider_cli {
  ($e:ty) => {
    impl<T> ConsiderCli<T> for Result<T, $e> {
      /// Изменяет параметры возможной ошибки на указанные.
      fn consider_cli(self, error_text_replacement: Option<String>) -> Result<T, CliError> {
        self.map_err(|e| {
          let mut new_error = CliError { message: e.to_string() };
          if error_text_replacement.is_some() {
            new_error.message = error_text_replacement.unwrap();
          }
          new_error
        })
      }
    }

    impl From<$e> for CliError {
      /// Создаёт `CliError` из данной ошибки.
      fn from(value: $e) -> Self {
        value.to_string().into()
      }
    }
  };
}

#[cfg(not(any(target_arch = "wasm32", target_arch = "wasm64")))]
impl_consider!(rmp_serde::encode::Error);
#[cfg(not(any(target_arch = "wasm32", target_arch = "wasm64")))]
impl_consider!(rmp_serde::decode::Error);
#[cfg(not(any(target_arch = "wasm32", target_arch = "wasm64")))]
impl_consider!(std::io::Error);
#[cfg(not(any(target_arch = "wasm32", target_arch = "wasm64")))]
impl_consider!(std::string::FromUtf8Error);
#[cfg(not(any(target_arch = "wasm32", target_arch = "wasm64")))]
impl_consider!(std::env::VarError);
#[cfg(not(any(target_arch = "wasm32", target_arch = "wasm64")))]
impl_consider!(std::sync::mpsc::RecvError);
#[cfg(not(any(target_arch = "wasm32", target_arch = "wasm64")))]
impl_consider!(serde_json::Error);
#[cfg(not(any(target_arch = "wasm32", target_arch = "wasm64")))]
impl_consider!(BoxDynError);
#[cfg(not(any(target_arch = "wasm32", target_arch = "wasm64")))]
impl_consider!(std::num::ParseIntError);
#[cfg(not(any(target_arch = "wasm32", target_arch = "wasm64")))]
impl_consider!(std::num::ParseFloatError);
#[cfg(not(any(target_arch = "wasm32", target_arch = "wasm64")))]
impl_consider!(tracing::subscriber::SetGlobalDefaultError);

#[cfg(feature = "salvo")]
#[cfg(not(any(target_arch = "wasm32", target_arch = "wasm64")))]
impl_consider!(salvo::Error);

#[cfg(feature = "salvo")]
#[cfg(not(any(target_arch = "wasm32", target_arch = "wasm64")))]
impl_consider!(salvo::hyper::http::status::InvalidStatusCode);

#[cfg(feature = "salvo")]
#[cfg(not(any(target_arch = "wasm32", target_arch = "wasm64")))]
impl_consider!(salvo::http::ParseError);

#[cfg(feature = "salvo")]
#[cfg(not(any(target_arch = "wasm32", target_arch = "wasm64")))]
impl<T> Consider<T> for Result<T, Option<&Box<dyn Any + Send + Sync>>> {
  /// Изменяет параметры возможной ошибки на указанные.
  fn consider(
    self,
    status_code: Option<StatusCode>,
    error_text_replacement: Option<impl Into<String>>,
    public: bool,
  ) -> Result<T, ErrorResponse> {
    self.map_err(|_| {
      let mut new_error = ErrorResponse {
        status_code,
        error_text: "Depot obtain failed!".into(),
        original_text: None,
        public_error: public,
      };
      if error_text_replacement.is_some() {
        new_error.original_text = Some(new_error.error_text.to_owned());
        new_error.error_text = error_text_replacement.unwrap().into();
      }
      new_error
    })
  }
}

#[cfg(feature = "salvo")]
#[cfg(not(any(target_arch = "wasm32", target_arch = "wasm64")))]
impl From<Option<&Box<dyn Any + Send + Sync>>> for ErrorResponse {
  /// Создаёт `ErrorResponse` из данной ошибки.
  fn from(_value: Option<&Box<(dyn Any + Send + Sync + 'static)>>) -> Self {
    "Depot obtain failed!".into()
  }
}

#[cfg(feature = "salvo")]
#[cfg(not(any(target_arch = "wasm32", target_arch = "wasm64")))]
impl<T, U> Consider<T> for Result<T, std::sync::mpsc::SendError<U>> {
  /// Изменяет параметры возможной ошибки на указанные.
  fn consider(
    self,
    status_code: Option<StatusCode>,
    error_text_replacement: Option<impl Into<String>>,
    public: bool,
  ) -> Result<T, ErrorResponse> {
    self.map_err(|e| {
      let mut new_error = ErrorResponse {
        status_code,
        error_text: e.to_string(),
        original_text: None,
        public_error: public,
      };
      if error_text_replacement.is_some() {
        new_error.original_text = Some(new_error.error_text.to_owned());
        new_error.error_text = error_text_replacement.unwrap().into();
      }
      new_error
    })
  }
}

#[cfg(feature = "salvo")]
#[cfg(not(any(target_arch = "wasm32", target_arch = "wasm64")))]
impl<U> From<std::sync::mpsc::SendError<U>> for ErrorResponse {
  /// Создаёт `ErrorResponse` из данной ошибки.
  fn from(value: std::sync::mpsc::SendError<U>) -> Self {
    value.to_string().into()
  }
}

#[cfg(feature = "salvo")]
#[cfg(not(any(target_arch = "wasm32", target_arch = "wasm64")))]
impl_consider!(salvo::http::header::ToStrError);

#[cfg(feature = "reqwest")]
#[cfg(not(any(target_arch = "wasm32", target_arch = "wasm64")))]
impl_consider!(reqwest::Error);

#[cfg(feature = "salvo")]
#[cfg(not(any(target_arch = "wasm32", target_arch = "wasm64")))]
impl_consider!(salvo::http::errors::StatusError);

#[cfg(any(target_arch = "wasm32", target_arch = "wasm64"))]
impl_consider_cli!(rmp_serde::encode::Error);
#[cfg(any(target_arch = "wasm32", target_arch = "wasm64"))]
impl_consider_cli!(rmp_serde::decode::Error);
#[cfg(any(target_arch = "wasm32", target_arch = "wasm64"))]
impl_consider_cli!(std::io::Error);
#[cfg(any(target_arch = "wasm32", target_arch = "wasm64"))]
impl_consider_cli!(std::string::FromUtf8Error);
#[cfg(any(target_arch = "wasm32", target_arch = "wasm64"))]
impl_consider_cli!(serde_json::Error);
#[cfg(any(target_arch = "wasm32", target_arch = "wasm64"))]
impl_consider_cli!(BoxDynError);

#[cfg(feature = "reqwest")]
#[cfg(any(target_arch = "wasm32", target_arch = "wasm64"))]
impl_consider_cli!(reqwest::Error);
