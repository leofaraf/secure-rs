#[cfg(feature = "compression")]
use secure::secure_str_brotli;

#[cfg(feature = "compression")]
secure_str_brotli!("brotli_value", "decompressed");

#[cfg(feature = "compression")]
#[test]
fn secure_str_brotli() {
    assert_eq!(get_brotli_value(), "decompressed".to_string());
}

#[cfg(feature = "encryption")]
mod encryption {
    use secure::secure_str_aes;

    secure_str_aes!("qaDYO1pU8Ud3ozV9r2kujgbXKFLObhx8tJWZvhhjttM=", "aes_value", "decrypted");

    #[test]
    pub fn include_secure_bytes_brotli() {
        assert_eq!(get_aes_value(), "decrypted");
    }
}