#[cfg(feature = "compression")]
use secure::include_secure_str_brotli;

#[cfg(feature = "compression")]
include_secure_str_brotli!("samples/loremipsum.txt", "brotli_value");

#[cfg(feature = "compression")]
#[test]
fn include_secure_str_brotli() {
    assert_eq!(get_brotli_value(), include_str!("../samples/loremipsum.txt"));
}