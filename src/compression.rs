//! Macros for compressing data sent by the `salvo` server.

/// Brotli compression salvo's hoop.
///
/// Usage:
///
/// ```rust
/// use salvo::Router;
/// use cc_utils::brotli;
///
/// let router = Router::with_hoop(brotli!()).path("new-compressed-json").get(hello_compressed_json);
/// ```
#[cfg(feature = "salvo")]
#[cfg(not(any(target_arch = "wasm32", target_arch = "wasm64")))]
#[macro_export]
macro_rules! brotli {
  () => {
    salvo::prelude::Compression::new()
      .disable_all()
      .enable_brotli(salvo::prelude::CompressionLevel::Minsize)
      .content_types(&[salvo::http::mime::APPLICATION_JSON])
      .force_priority(true)
      .min_length(10)
  };
}
