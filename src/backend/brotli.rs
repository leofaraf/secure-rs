use std::io::Write;

use base64::{prelude::BASE64_STANDARD, Engine};

/// Accepts raw string, returns BASE64 string
/// 
/// Decode via:
/// ```rust
/// use base64::{prelude::BASE64_STANDARD, Engine};
/// use std::io::Write;
/// 
/// fn decode_brotli(encoded_value: String) -> String {
///     let mut decompressed_data = Vec::new();
///     {
///         let data = BASE64_STANDARD.decode(encoded_value.as_bytes()).unwrap_or_default();
///         let mut decompressor = brotli::DecompressorWriter::new(&mut decompressed_data, 4096);
///         decompressor.write_all(&data).unwrap();
///     }
///     String::from_utf8(decompressed_data).unwrap()
/// }
/// ```
pub fn encode_brotli(value: &[u8]) -> String {
    let mut compressed_data = Vec::new();
    {
        let mut compressor = brotli::CompressorWriter::new(&mut compressed_data, 4096, 0, 0);
        compressor.write_all(value).unwrap();
    }
    BASE64_STANDARD.encode(compressed_data)
}