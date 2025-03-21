#[cfg(feature = "compression")]
mod compression {
    use secure::secure_str_brotli;
    
    secure_str_brotli!("brotli_value", "decompressed");

    #[test]
    fn secure_str_brotli() {
        assert_eq!(get_brotli_value(), "decompressed".to_string());
    }
}


#[cfg(feature = "encryption")]
mod encryption {
    use secure::secure_str_aes;

    secure_str_aes!("Thisi$MyKeyT0Encryp!thislastTime", "aes_value", "decrypted");

    #[test]
    pub fn secure_str_aes() {
        assert_eq!(get_aes_value(), "decrypted");
    }
}