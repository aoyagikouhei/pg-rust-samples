use chrono::prelude::*;
use rand::distr::Alphanumeric;
use rand::{rng, Rng};
use std::collections::HashMap;
use std::sync::LazyLock;
use unicode_normalization::UnicodeNormalization;
use unicode_segmentation::UnicodeSegmentation;
use uuid::{NoContext, Timestamp, Uuid};

pub fn random_string(length: usize) -> String {
    rng()
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

pub fn uuid_to_utc(uuid: &Uuid) -> Option<DateTime<Utc>> {
    uuid.get_timestamp().and_then(|it| {
        let (secs, nsecs) = it.to_unix();
        Utc.timestamp_opt(secs as i64, nsecs).single()
    })
}

pub fn utc_to_uuid(utc: Option<DateTime<Utc>>) -> Uuid {
    if let Some(utc) = utc {
        let ts = Timestamp::from_unix(
            NoContext,
            utc.timestamp() as u64,
            utc.timestamp_subsec_nanos(),
        );
        Uuid::new_v7(ts)
    } else {
        Uuid::now_v7()
    }
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

    // RUST_LOG=info REALM_CODE=test cargo test -p common test_common_get_utc -- --nocapture --test-threads=1
    #[tokio::test]
    async fn test_common_get_utc() -> anyhow::Result<()> {
        let src = Utc.timestamp_opt(1497624119, 123_999_999).single().unwrap();
        let uuid = utc_to_uuid(Some(src));
        let dst: DateTime<Utc> = uuid_to_utc(&uuid).unwrap();
        assert_eq!(dst.timestamp(), 1497624119);

        // ミリ秒以下切り捨て
        assert_eq!(dst.timestamp_subsec_nanos(), 123_000_000);
        Ok(())
    }
}
