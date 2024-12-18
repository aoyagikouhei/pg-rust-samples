use std::time::Duration;

use futures_util::TryStreamExt;
use sqlx::{postgres::PgListener, PgPool};

const CHANNEL_MSG: &str = "channel_msg";
const CHANNEL_TERM: &str = "channel_term";

pub async fn execute(pool: &PgPool) -> Result<(), sqlx::Error> {
    let mut listener = PgListener::connect_with(&pool).await?;

    let notify_pool = pool.clone();
    let _t = tokio::spawn(async move {
        tokio::time::sleep(Duration::from_secs(2)).await;
        let _ = notfiy(&notify_pool, CHANNEL_MSG, "hello").await;
        tokio::time::sleep(Duration::from_secs(2)).await;
        let _ = notfiy(&notify_pool, CHANNEL_MSG, "world").await;
        tokio::time::sleep(Duration::from_secs(2)).await;
        let _ = notfiy(&notify_pool, CHANNEL_TERM, "goodbye").await;
    });

    listener.listen_all(vec![CHANNEL_MSG, CHANNEL_TERM]).await?;
    let mut stream = listener.into_stream();
    loop {
        match stream.try_next().await {
            Ok(Some(notification)) => {
                println!("[from stream]: {notification:?}");
                if notification.channel() == CHANNEL_TERM {
                    break;
                }
            }
            Ok(None) => {
                println!("Stream closed.");
                break;
            }
            Err(e) => {
                eprintln!("Error: {e}");
                break;
            }
        }
    }
    Ok(())
}

async fn notfiy(pool: &PgPool, channel: &str, payload: &str) -> Result<(), sqlx::Error> {
    let sql = r#"SELECT pg_notify($1, $2)"#;
     sqlx::query(sql).bind(channel).bind(payload).execute(pool).await?;
     Ok(())
 }
 