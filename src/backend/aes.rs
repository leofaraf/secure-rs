use aes_gcm::{aead::{Aead, OsRng}, AeadCore, Aes256Gcm, KeyInit};
use base64::{prelude::BASE64_STANDARD, Engine};

/// Accepts raw string, returns BASE64 string
/// 
/// Decode via:
/// ```rust
/// use base64::{prelude::BASE64_STANDARD, Engine};
/// use aes_gcm::{aead::{Aead, OsRng}, AeadCore, Aes256Gcm, KeyInit};
/// 
/// fn decode_aes(key: &[u8], value: &[u8]) -> String {
///     let decoded_data = {
///         let key = Aes256Gcm::new_from_slice(key).expect("Can't create key from slice");
///         let nonce = Aes256Gcm::generate_nonce(&mut OsRng); 
///     
///         key.decrypt(&nonce, value).unwrap()
///     };
/// 
///     String::from_utf8(decoded_data).unwrap()
/// }
/// ```
pub fn encode_aes(key: &[u8], value: &[u8]) -> String {
    let encoded_data = {
        let key = Aes256Gcm::new_from_slice(key).expect("Can't create key from slice");
        let nonce = Aes256Gcm::generate_nonce(&mut OsRng); 
        key.encrypt(&nonce, value).unwrap()
    };

    BASE64_STANDARD.encode(encoded_data)
}