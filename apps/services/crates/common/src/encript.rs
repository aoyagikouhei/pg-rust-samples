use aes::cipher::StreamCipher;
use ctr::cipher::KeyIvInit;
type Aes128Ctr64LE = ctr::Ctr64LE<aes::Aes128>;
use rand::{Rng, SeedableRng};

// 暗号化
pub fn encrypt(key: &str, data: &str) -> Vec<u8> {
    let mut rng = rand::rngs::StdRng::from_entropy();
    let iv = rng.gen::<[u8; 16]>();
    let mut cipher = Aes128Ctr64LE::new(key.as_bytes().into(), &iv.into());
    let mut buf = data.as_bytes().to_vec();
    cipher.apply_keystream(&mut buf);
    let mut res = iv.to_vec();
    res.extend(buf);
    res
}

// 復号化
pub fn decrypt(key: &str, data: &[u8]) -> String {
    let mut cipher = Aes128Ctr64LE::new(key.as_bytes().into(), data[0..16].into());
    let mut buf2: Vec<u8> = vec![0; data.len() - 16];
    if let Err(err) = cipher.apply_keystream_b2b(&data[16..], buf2.as_mut_slice()) {
        println!("{}", err);
    }
    String::from_utf8(buf2.to_vec()).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    // RUST_LOG=info REALM_CODE=test cargo test -p common test_common_encript -- --nocapture --test-threads=1
    #[tokio::test]
    async fn test_common_encript() -> anyhow::Result<()> {
        let plaintext = "予定表～①💖ﾊﾝｶｸだ";
        let key = "0123456701234567";
        let enc = encrypt(key, plaintext);
        let dec = decrypt(key, &enc);
        assert_eq!(plaintext, dec);
        println!("{}", dec);
        Ok(())
    }
}
