#[cfg(feature = "compression")]
mod compression {
    use secure::include_secure_str_brotli;
    
    include_secure_str_brotli!("samples/loremipsum.txt", "brotli_value");
    
    #[test]
    fn include_secure_str_brotli() {
        assert_eq!(get_brotli_value(), include_str!("../samples/loremipsum.txt"));
    }
}