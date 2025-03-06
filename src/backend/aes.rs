use aes_gcm::{aead::{Aead, OsRng}, AeadCore, Aes256Gcm, KeyInit};

pub fn encode_aes(key: &[u8], value: &[u8]) -> Vec<u8> {
    let key = Aes256Gcm::new_from_slice(key).expect("Can't create key from slice");
    let nonce = Aes256Gcm::generate_nonce(&mut OsRng); 

    key.encrypt(&nonce, value).unwrap()
}