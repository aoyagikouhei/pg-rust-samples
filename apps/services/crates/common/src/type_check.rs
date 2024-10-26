use std::{net::{IpAddr, Ipv4Addr}, str::FromStr};
use bit_vec::BitVec;
use chrono::prelude::*;
use serde_json::json;
use sqlx::{postgres::types::{Oid, PgHstore}, prelude::*, types::mac_address::MacAddress, Pool, Postgres};
use uuid::Uuid;
use rust_decimal::Decimal;

const SQL: &str = r#"
SELECT
    $1::BOOL AS bool_val
    ,$2::BOOL[] AS bool_array_val
    ,$3::BOOL AS bool_option_some_val
    ,$4::BOOL AS bool_option_none_val
    ,$5::"char" AS char_val
    ,$6::SMALLINT AS smallint_val
    ,$7::INT AS int_val
    ,$8::OID AS oid_val
    ,$9::BIGINT AS bigint_val
    ,$10::REAL AS real_val
    ,$11::DOUBLE PRECISION AS double_val
    ,$12::TEXT AS text_val
    ,$13::BYTEA AS bytes_val
    ,$14::INET AS inet_val
    ,$15::TIMESTAMP AS timestamp_val
    ,$16::TIMESTAMPTZ AS timestamptz_val
    ,$17::DATE AS date_val
    ,$18::TIME AS time_val
    ,$19::MACADDR AS macaddr_val
    ,$20::JSONB AS jsonb_val
    ,$21::UUID AS uuid_val
    ,$22::VARBIT AS varbit_val
    ,$23::NUMERIC AS decimal_val
    ,$24::HSTORE AS hstore_val
"#;

#[derive(FromRow, Debug, PartialEq)]
struct Data {
    bool_val: bool,
    bool_array_val: Vec<bool>,
    bool_option_some_val: Option<bool>,
    bool_option_none_val: Option<bool>,
    char_val: i8,
    smallint_val: i16,
    int_val: i32,
    oid_val: Oid,
    bigint_val: i64,
    real_val: f32,
    double_val: f64,
    text_val: String,
    bytes_val: Vec<u8>,
    inet_val: IpAddr,
    timestamp_val: NaiveDateTime,
    timestamptz_val: DateTime<Utc>,
    date_val: NaiveDate,
    time_val: NaiveTime,
    macaddr_val: MacAddress,
    jsonb_val: serde_json::Value,
    uuid_val: Uuid,
    varbit_val: BitVec,
    decimal_val: Decimal,
    hstore_val: PgHstore,
}

pub async fn execute(pool: &Pool<Postgres>) -> Result<(), sqlx::Error> {
    let data = Data {
        bool_val: true,
        bool_array_val: vec![true, false],
        bool_option_some_val: Some(true),
        bool_option_none_val: None,
        char_val: 1,
        smallint_val: 2,
        int_val: 3,
        oid_val: Oid(4),
        bigint_val: 5,
        real_val: 6.1,
        double_val: 7.1,
        text_val: "予定表～①💖ﾊﾝｶｸだ".to_string(),
        bytes_val: vec![240, 159, 146, 150],
        inet_val: IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)),
        timestamp_val: NaiveDateTime::from_str("2001-02-03T04:05:06").unwrap(),
        timestamptz_val: Utc.with_ymd_and_hms(2001, 2, 3, 4, 5, 6).unwrap(),
        date_val: NaiveDate::from_ymd_opt(2002, 3, 4).unwrap(),
        time_val: NaiveTime::from_hms_milli_opt(8, 59, 59, 100).unwrap(),
        macaddr_val: MacAddress::new([0x12, 0x34, 0x56, 0xAB, 0xCD, 0xEF]),
        jsonb_val: json!({
                "name" : "予定表～①💖ﾊﾝｶｸだ",
                "age" : 99
            }),
        uuid_val: Uuid::new_v4(),
        varbit_val: {
            let mut varbit_val = BitVec::from_elem(10, false);
            varbit_val.set(2, true);
            varbit_val
        },
        decimal_val: Decimal::from_str("1.1").unwrap(),
        hstore_val: {
            let mut val = PgHstore::default();
            val.insert("key".to_owned(), Some("value".to_owned()));
            val
        }
    };

    let res: Data = sqlx::query_as(SQL)
        .bind(&data.bool_val)
        .bind(&data.bool_array_val)
        .bind(&data.bool_option_some_val)
        .bind(&data.bool_option_none_val)
        .bind(&data.char_val)
        .bind(&data.smallint_val)
        .bind(&data.int_val)
        .bind(&data.oid_val)
        .bind(&data.bigint_val)
        .bind(&data.real_val)
        .bind(&data.double_val)
        .bind(&data.text_val)
        .bind(&data.bytes_val)
        .bind(&data.inet_val)
        .bind(&data.timestamp_val)
        .bind(&data.timestamptz_val)
        .bind(&data.date_val)
        .bind(&data.time_val)
        .bind(&data.macaddr_val)
        .bind(&data.jsonb_val)
        .bind(&data.uuid_val)
        .bind(&data.varbit_val)
        .bind(&data.decimal_val)
        .bind(&data.hstore_val)
        .fetch_one(pool).await?;
    assert_eq!(data, res);
    println!("{:?}", data);

    Ok(())
}
