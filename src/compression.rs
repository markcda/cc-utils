//! Macros for compressing data sent by the `salvo` server.

/// Usage:
///
/// ```rust
/// use salvo::Router;
/// use cc_utils::brotli;
///
/// let router = Router::with_hoop(brotli!()).path("new-compressed-json").get(hello_compressed_json);
/// ```
#[cfg(feature = "salvo")]
#[macro_export]
macro_rules! brotli { () => {
  Compression::new()
    .disable_all()
    .enable_brotli(CompressionLevel::Minsize)
    .content_types(&[salvo::http::mime::APPLICATION_JSON])
    .force_priority(true)
    .min_length(10)
  };
}
