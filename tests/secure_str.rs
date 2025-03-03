#[cfg(feature = "compression")]
use secure::secure_str_brotli;

#[cfg(feature = "compression")]
secure_str_brotli!("brotli_value", "decompressed");

#[cfg(feature = "compression")]
#[test]
fn secure_str_brotli() {
    assert_eq!(get_brotli_value(), "decompressed".to_string());
}