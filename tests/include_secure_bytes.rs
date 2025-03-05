#[cfg(feature = "compression")]
mod compression {
    use secure::include_secure_bytes_brotli;

    include_secure_bytes_brotli!("samples/112233.bin", "bytes");

    #[test]
    pub fn include_secure_bytes_brotli() {
        assert_eq!(get_bytes(), include_bytes!("../samples/112233.bin"));
    }
}