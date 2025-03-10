
use base64::{prelude::BASE64_STANDARD, Engine};
use byte_aes::Aes256Cryptor;

/// Accepts raw string, returns BASE64 string
/// 
/// Decode via:
/// ```rust
/// use byte_aes::Aes256Cryptor;
/// 
/// pub fn decode_aes(key: &str, value: &[u8]) -> String {
///     let decoded_data = {
///         let cryptor = Aes256Cryptor::try_from(key).unwrap();
///         cryptor.decrypt(value).expect("Can't decrypt data")
///     };
/// 
///     String::from_utf8(decoded_data).unwrap()
/// }
/// ```
pub fn encode_aes(key: &str, value: &[u8]) -> String {
    let encoded_data = {
        let cryptor = Aes256Cryptor::try_from(key).unwrap();
        cryptor.encrypt(value)
    };

    BASE64_STANDARD.encode(encoded_data)
}