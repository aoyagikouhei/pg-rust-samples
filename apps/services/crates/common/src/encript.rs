use base64::prelude::*;
use chacha20poly1305::aead::Aead;
use chacha20poly1305::{AeadCore, KeyInit, XChaCha20Poly1305};
use rand::rngs::OsRng;

#[derive(Debug, thiserror::Error)]
pub enum EncryptError {
    #[error("Base64 {0}")]
    Base64(#[from] base64::DecodeError),

    #[error("Utf8 {0}")]
    Utf8(#[from] std::string::FromUtf8Error),

    #[error("Chacha20 {0}")]
    Chacha20(#[from] chacha20poly1305::Error),
}

// 暗号化
pub fn encrypt_with_base64(key: &str, data: &str) -> Result<String, EncryptError> {
    let res = encrypt(key.as_bytes(), data.as_bytes())?;
    Ok(BASE64_STANDARD.encode(res))
}

// 復号化
pub fn decrypt_with_base64(key: &str, data: &str) -> Result<String, EncryptError> {
    let data = BASE64_STANDARD.decode(data)?;
    let plaintext = decrypt(key.as_bytes(), &data)?;
    String::from_utf8(plaintext.to_vec()).map_err(|e| e.into())
}

// 暗号化
pub fn encrypt(key: &[u8], data: &[u8]) -> Result<Vec<u8>, EncryptError> {
    let cipher = XChaCha20Poly1305::new(key.into());
    let nonce = XChaCha20Poly1305::generate_nonce(&mut OsRng); // 192-bits; unique per message
    let ciphertext = cipher.encrypt(&nonce, data)?;
    let mut res = nonce.to_vec();
    res.extend(ciphertext);
    Ok(res)
}

// 復号化
pub fn decrypt(key: &[u8], data: &[u8]) -> Result<Vec<u8>, EncryptError> {
    let nonce = &data[0..24];
    let cipher = XChaCha20Poly1305::new(key.into());
    let plaintext = cipher.decrypt(nonce.into(), &data[24..])?;
    Ok(plaintext.to_vec())
}

#[cfg(test)]
mod tests {
    use super::*;

    // RUST_LOG=info REALM_CODE=test cargo test -p common test_common_encript_chacha20 -- --nocapture --test-threads=1
    #[tokio::test]
    async fn test_common_encript_chacha20() -> anyhow::Result<()> {
        let plaintext = "予定表～①💖ﾊﾝｶｸだ";
        let key = "01234567012345670123456701234567"; // 32byte
        let enc = encrypt_with_base64(key, plaintext)?;
        let dec = decrypt_with_base64(key, &enc)?;
        assert_eq!(plaintext, dec);
        println!("{},{}", enc, dec);
        Ok(())
    }
}
