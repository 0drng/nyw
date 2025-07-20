use sqlx::{Pool, Sqlite};

use crate::model::application::Application;

pub async fn save_applications(pool: &Pool<Sqlite>, applications: Vec<Application>) -> Result<(), sqlx::Error> {
    for application in applications {
        sqlx::query!(
            r#"
            INSERT INTO applications (id, name, action)
            VALUES (?, ?, ?)
            "#,
            application.id,
            application.name,
            application.action
        )
        .execute(pool)
        .await?;
    }

    return Ok(());
}

pub async fn get_applications_by_lock_id(pool: &Pool<Sqlite>, lock_id: i64) -> Result<Vec<Application>, sqlx::Error> {
    return sqlx::query_as!(
        Application,
        r#"
        SELECT id as "id!:u32", name, action
        FROM applications 
        WHERE id = ?
        "#,
        lock_id
    ).fetch_all(pool)
    .await;
}

pub async fn get_applications_by_lock_id_and_action(pool: &Pool<Sqlite>, lock_id: u32, action: String) -> Result<Vec<Application>, sqlx::Error> {
    return sqlx::query_as!(
        Application,
        r#"
        SELECT id as "id!:u32", name, action
        FROM applications 
        WHERE id = ?
        AND action = ?
        "#,
        lock_id,
        action
    ).fetch_all(pool)
    .await;
} 