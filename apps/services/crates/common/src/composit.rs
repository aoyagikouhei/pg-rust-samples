use sqlx::{prelude::*, Pool, Postgres};
const SQL: &str = r#"
SELECT
    t1.*
FROM
    sample_get_select_composite(
        p_value := $1
    ) AS t1 
"#;

#[derive(Debug, sqlx::Type)]
#[sqlx(type_name = "type_enum_os", rename_all = "snake_case")]
enum TypeEnumOs {
    Linux,
    MacOs,
    Windows,
}

#[derive(sqlx::Type, Debug)]
#[sqlx(transparent)]
struct DomainInteger(i64);

#[derive(sqlx::Type, Debug)]
#[sqlx(transparent)]
struct MemorySizeUnit(i64);

#[derive(FromRow, Debug, sqlx::Type)]
#[sqlx(type_name = "type_sample_get_select_composite")]
struct TypePgGetSelectComposite {
    os: TypeEnumOs,
    cpu_count: DomainInteger,
    memory_size: MemorySizeUnit,
}

pub async fn execute(pool: &Pool<Postgres>) -> Result<(), sqlx::Error> {
    let data = TypePgGetSelectComposite {
        os: TypeEnumOs::MacOs,
        cpu_count: DomainInteger(2),
        memory_size: MemorySizeUnit(1024),
    };

    let row: Vec<TypePgGetSelectComposite> =
        sqlx::query_as(SQL).bind(&data).fetch_all(pool).await?;

    println!("{:?}", row);
    Ok(())
}
