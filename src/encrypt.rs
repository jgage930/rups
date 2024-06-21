use anyhow::Result;
use magic_crypt::{new_magic_crypt, MagicCryptTrait};
use std::fs;

pub fn read_key(path: &str) -> String {
    fs::read_to_string(path).expect("Unable to read key file")
}

pub fn encrypt(key: &str, content: &Vec<u8>) -> Vec<u8> {
    let mcrypt = new_magic_crypt!(key, 256);
    mcrypt.encrypt_bytes_to_bytes(&content)
}

pub fn decrypt(key: &str, content: &Vec<u8>) -> Result<Vec<u8>> {
    let mcrypt = new_magic_crypt!(key, 256);
    Ok(mcrypt.decrypt_bytes_to_bytes(&content)?)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encryption() {
        let message = "my secret message".as_bytes().to_vec();

        let key = read_key("key");
        let encrypted_message = encrypt(&key, &message);
        let unencrypted_message = decrypt(&key, &encrypted_message).unwrap();

        assert_eq!(message, unencrypted_message);
    }
}
