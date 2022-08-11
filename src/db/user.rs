use sqlx::PgPool;
use uuid::Uuid;

use crate::models::{
    ServiceError,
    StatusCode,
    NewUser
};

pub async fn insert_user(user: &NewUser, db_pool: &PgPool) -> Result<Uuid, ServiceError> {
    let id = uuid::Uuid::new_v4();

    sqlx::query!(
            "INSERT INTO users
            VALUES ($1, $2, $3)",
            id,
            user.login,
            user.password
        ).execute(db_pool)
        .await
        .map_err(|e| ServiceError { status_code: StatusCode::InternalError, detail: Some(e.to_string())})?;

    Ok(id)
}

pub async fn select_user_id(login: &str, password: &str, db_pool: &PgPool) -> Result<Option<Uuid>, ServiceError> {
    let result = sqlx::query!(
            "SELECT id
            FROM users
            WHERE login = $1 AND password = $2
            LIMIT 1",
            login,
            password
        )
        .fetch_all(db_pool)
        .await
        .map_err(|e| ServiceError { status_code: StatusCode::InternalError, detail: Some(e.to_string())})?;

    Ok(result.first().map(|r| r.id))
}

pub async fn is_user_exist(login: &String, db_pool: &PgPool) -> std::result::Result<bool, ServiceError> {
    let result = sqlx::query!(
            "SELECT COUNT (*) as count
            FROM users
            WHERE login = $1 ",
            login
        )
        .fetch_one(db_pool)
        .await
        .map_err(|e| ServiceError { status_code: StatusCode::InternalError, detail: Some(e.to_string())})?;

    Ok(result.count.unwrap() > 0)
}