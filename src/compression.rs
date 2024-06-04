//! Макросы для сжатия отдаваемых сервером `salvo` данных.

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
