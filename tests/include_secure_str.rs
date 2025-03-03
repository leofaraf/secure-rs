#[cfg(feature = "brotli")]
use secure::include_secure_str_brotli;

#[cfg(feature = "brotli")]
include_secure_str_brotli!("samples/loremipsum.txt", "brotli_value");

#[cfg(feature = "brotli")]
#[test]
fn include_secure_str_brotli() {
    assert_eq!(get_brotli_value(), include_str!("../samples/loremipsum.txt"));
}