use secure::secure_str_brotli;

secure_str_brotli!("brotli_value", "decompressed");

#[test]
fn secure_str_brotli() {
    assert_eq!(get_brotli_value(), "decompressed".to_string());
}