#[cfg(feature = "compression")]
mod compression {
    use secure::include_secure_bytes_brotli;

    include_secure_bytes_brotli!("samples/112233.bin", "bytes");

    #[test]
    pub fn include_secure_bytes_brotli() {
        assert_eq!(get_bytes(), include_bytes!("../samples/112233.bin"));
    }
}

// #[cfg(feature = "encryption")]
// mod encryption {
//     use secure::secure_str_aes;

//     secure_str_aes!("Thisi$MyKeyT0Encryp!thislastTime", "aes_value");

//     #[test]
//     pub fn secure_str_aes() {
//         assert_eq!(get_aes_value(), "decrypted");
//     }
// }