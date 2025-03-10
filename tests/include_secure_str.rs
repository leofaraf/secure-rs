#[cfg(feature = "compression")]
mod compression {
    use secure::include_secure_str_brotli;
    
    include_secure_str_brotli!("samples/loremipsum.txt", "brotli_value");
    
    #[test]
    fn include_secure_str_brotli() {
        assert_eq!(get_brotli_value(), include_str!("../samples/loremipsum.txt"));
    }
}

#[cfg(feature = "encryption")]
mod encryption {
    use secure::include_secure_str_aes;

    include_secure_str_aes!("samples/loremipsum.txt", "Thisi$MyKeyT0Encryp!thislastTime", "aes_value");

    #[test]
    pub fn include_secure_str_aes() {
        assert_eq!(get_aes_value(), include_str!("../samples/loremipsum.txt"));
    }
}