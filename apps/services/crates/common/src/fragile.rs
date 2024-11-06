use sqlx::{prelude::*, Pool, Postgres};
use uuid::Uuid;

#[derive(Debug, sqlx::Type, Clone)]
#[sqlx(type_name = "type_user_kbn")]
enum UserKbn {
    Admin,
    Normal,
}

#[allow(dead_code)]
#[derive(FromRow, Debug)]
struct User {
    pub uuid: Uuid,
    pub user_name: String,
    pub user_email: String,
    pub user_kbn: UserKbn,
}

impl User {
    pub async fn insert(
        pool: &Pool<Postgres>,
        user_name: &str,
        user_email: &str,
        user_kbn: &UserKbn,
    ) -> Result<(), sqlx::Error> {
        let uuid: Uuid = Uuid::now_v7();
        sqlx::query("INSERT INTO public.users (uuid, user_name, user_email, user_kbn) VALUES ($1, $2, $3, $4)")
            .bind(&uuid)
            .bind(user_name)
            .bind(user_email)
            .bind(user_kbn)
            .execute(pool)
            .await?;
        Ok(())
    }

    pub async fn all(pool: &Pool<Postgres>) -> Result<Vec<User>, sqlx::Error> {
        let res: Vec<User> = sqlx::query_as("SELECT * FROM public.users")
            .fetch_all(pool)
            .await?;
        Ok(res)
    }

    pub async fn delete_all(pool: &Pool<Postgres>) -> Result<(), sqlx::Error> {
        sqlx::query("DELETE FROM public.users")
            .execute(pool)
            .await?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::User;

    async fn setup() -> anyhow::Result<sqlx::Pool<sqlx::Postgres>> {
        let pool = sqlx::postgres::PgPoolOptions::new()
            .max_connections(5)
            .connect("postgres://user:pass@postgresql/web")
            .await?;
        User::delete_all(&pool).await?;
        Ok(pool)
    }

    // cargo test test_fragile_each_column -- --nocapture --test-threads=1
    #[tokio::test]
    async fn test_fragile_each_column() -> anyhow::Result<()> {
        let pool = setup().await?;
        User::insert(&pool, "taro", "taro@example.com", &super::UserKbn::Normal).await?;
        let users = User::all(&pool).await?;
        assert_eq!(users.len(), 1);
        Ok(())
    }
}
