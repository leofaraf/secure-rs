#[cfg(feature = "brotli")]
use secure::secure_str_brotli;

#[cfg(feature = "brotli")]
secure_str_brotli!("brotli_value", "decompressed");

#[cfg(feature = "brotli")]
#[test]
fn secure_str_brotli() {
    assert_eq!(get_brotli_value(), "decompressed".to_string());
}