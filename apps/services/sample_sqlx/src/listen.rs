use std::{pin::pin, sync::atomic::{AtomicI64, Ordering}, time::Duration};

use futures_util::TryStreamExt;
use sqlx::{postgres::PgListener, prelude::*, PgPool, Pool, Postgres};

const LISTEN_DURATION: Duration = Duration::from_secs(5);

pub async fn execute(pool: &Pool<Postgres>) -> Result<(), sqlx::Error> {
    let mut listener = PgListener::connect_with(&pool).await?;

    let notify_pool = pool.clone();
    let _t = tokio::spawn(async move {
        let mut interval = tokio::time::interval(Duration::from_secs(2));

        while !notify_pool.is_closed() {
            interval.tick().await;
            notify(&notify_pool).await;
        }
    });

    println!("Starting LISTEN loop.");

    listener.listen_all(vec!["chan0", "chan1", "chan2"]).await?;

    let mut counter = 0usize;
    loop {
        let notification = listener.recv().await?;
        println!("[from recv]: {notification:?}");

        counter += 1;
        if counter >= 3 {
            break;
        }
    }

    // Prove that we are buffering messages by waiting for 6 seconds
    listener.execute("SELECT pg_sleep(6)").await?;

    let mut stream = listener.into_stream();

    // `Sleep` must be pinned
    let mut timeout = pin!(tokio::time::sleep(LISTEN_DURATION));

    loop {
        tokio::select! {
            res = stream.try_next() => {
                if let Some(notification) = res? {
                    println!("[from stream]: {notification:?}");
                } else {
                    break;
                }
            },
            _ = timeout.as_mut() => {
                // Don't run forever
                break;
            }
        }
    }

    pool.close().await;
    Ok(())
}

async fn notify(pool: &PgPool) {
    static COUNTER: AtomicI64 = AtomicI64::new(0);

    // There's two ways you can invoke `NOTIFY`:
    //
    // 1: `NOTIFY <channel>, '<payload>'` which cannot take bind parameters and
    // <channel> is an identifier which is lowercased unless double-quoted
    //
    // 2: `SELECT pg_notify('<channel>', '<payload>')` which can take bind parameters
    // and <channel> preserves its case
    //
    // We recommend #2 for consistency and usability.

    // language=PostgreSQL
    let res = sqlx::query(
        r#"
-- this emits '{ "payload": N }' as the actual payload
select pg_notify(chan, json_build_object('payload', payload)::text)
from (
         values ('chan0', $1),
                ('chan1', $2),
                ('chan2', $3)
     ) notifies(chan, payload)
    "#,
    )
    .bind(&COUNTER.fetch_add(1, Ordering::SeqCst))
    .bind(&COUNTER.fetch_add(1, Ordering::SeqCst))
    .bind(&COUNTER.fetch_add(1, Ordering::SeqCst))
    .execute(pool)
    .await;

    println!("[from notify]: {res:?}");
}