use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use std::collections::HashMap;
use std::sync::LazyLock;
use unicode_normalization::UnicodeNormalization;
use unicode_segmentation::UnicodeSegmentation;

pub fn random_string(length: usize) -> String {
    thread_rng()
        .sample_iter(&Alphanumeric)
        .take(length)
        .map(char::from)
        .collect()
}

pub fn normalize_string(src: &str) -> String {
    src.nfkc().collect::<String>()
}

static MAP: LazyLock<HashMap<&'static str, &'static str>> = LazyLock::new(|| {
    let mut map: HashMap<&'static str, &'static str> = HashMap::new();
    map.insert("Ａ", "A");
    map.insert("１", "1");
    map.insert("＃", "#");
    map.insert("＠", "@");
    map.insert("ｱ", "ア");
    map.insert("ｰ", "ー");
    map.insert("ｶﾞ", "ガ");
    map
});

pub fn special_normalize_string(src: &str) -> String {
    src.graphemes(true)
        .map(|c| match MAP.get(c) {
            Some(v) => *v,
            None => c,
        })
        .collect::<String>()
}

#[cfg(test)]
mod tests {
    use super::*;

    // RUST_LOG=info REALM_CODE=test cargo test -p common test_common_random_string -- --nocapture --test-threads=1
    #[tokio::test]
    async fn test_common_random_string() -> anyhow::Result<()> {
        let data = random_string(10);
        println!("{}", data);
        assert_eq!(data.len(), 10);
        Ok(())
    }

    // RUST_LOG=info REALM_CODE=test cargo test -p common test_common_normalize_string -- --nocapture --test-threads=1
    #[tokio::test]
    async fn test_common_normalize_string() -> anyhow::Result<()> {
        let src = "Ａ１＃＠（ｱｶﾞｰ";
        let dst = "A1#@(アガー";
        assert_eq!(normalize_string(src), dst);
        Ok(())
    }

    // RUST_LOG=info REALM_CODE=test cargo test -p common test_common_special_normalize_string -- --nocapture --test-threads=1
    #[tokio::test]
    async fn test_common_special_normalize_string() -> anyhow::Result<()> {
        let src = "Ａ１＃＠（ｱｶﾞｰ";
        let dst = "A1#@（アガー";
        assert_eq!(special_normalize_string(src), dst);
        Ok(())
    }
}
