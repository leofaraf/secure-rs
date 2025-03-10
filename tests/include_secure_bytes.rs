#[cfg(feature = "compression")]
mod compression {
    use secure::include_secure_bytes_brotli;

    include_secure_bytes_brotli!("samples/image.png", "bytes");

    #[test]
    pub fn include_secure_bytes_brotli() {
        assert_eq!(get_bytes(), include_bytes!("../samples/image.png"));
    }
}

#[cfg(feature = "encryption")]
mod encryption {
    use secure::include_secure_bytes_aes;

    include_secure_bytes_aes!("samples/image.png", "Thisi$MyKeyT0Encryp!thislastTime", "aes_value");

    #[test]
    pub fn include_secure_bytes_aes() {
        assert_eq!(get_aes_value(), include_bytes!("../samples/image.png"));
    }
}