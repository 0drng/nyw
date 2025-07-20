use sqlx::{sqlite::SqliteConnectOptions, Pool, Sqlite, SqlitePool};

use crate::{error::ApplicationError, service::{file_service, language_service::Labels}};

pub async fn get_pool() -> Result<Pool<Sqlite>, ApplicationError> {
    let options = SqliteConnectOptions::new()
        .filename(format!("{}/lock.db", file_service::get_platform_specific_path()))
        .create_if_missing(true);

    let pool: Pool<Sqlite> = match SqlitePool::connect_with(options).await {
        Ok(pool) => pool,
        Err(e) => return Err(ApplicationError::new(Labels::Error_GetPoolError, Option::Some(vec![e.to_string()]))),
    };
    
    sqlx::migrate!().run(&pool).await.unwrap();

    return Ok(pool);
}