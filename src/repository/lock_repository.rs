use sqlx::{Pool, Sqlite};
use chrono::NaiveDateTime;

use crate::model::lock::{Lock, LockAdd};

pub async fn get_latest_lock_id(pool: &Pool<Sqlite>) -> Result<Option<u32>, sqlx::Error> {
    return sqlx::query_scalar::<_, u32>(
        r#"
            SELECT id 
            FROM locks 
            ORDER BY timestamp 
            DESC LIMIT 1
        "#,
    )
    .fetch_optional(pool)
    .await;
}

pub async fn get_lock_by_id(pool: &Pool<Sqlite>, id: i64) -> Result<Option<Lock>, sqlx::Error> {
    return sqlx::query_as!(
        Lock,
        r#"
        SELECT id as "id!:u32", hash, timestamp as "timestamp!:NaiveDateTime"
        FROM locks 
        WHERE id = ?
        "#,
        id
    )
    .fetch_optional(pool)
    .await;
}

pub async fn save_lock(pool: &Pool<Sqlite>, lock: LockAdd) -> Result<Lock, sqlx::Error> {
    let result = sqlx::query!(
        r#"
        INSERT INTO locks (hash)
        VALUES (?)
        "#,
        lock.hash
    )
    .execute(pool)
    .await?;
    
    let inserted_lock = get_lock_by_id(pool, result.last_insert_rowid()).await?.unwrap();

    return Ok(inserted_lock);
}