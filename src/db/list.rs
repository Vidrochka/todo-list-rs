use sqlx::PgPool;
use uuid::Uuid;

use crate::models::{
    ServiceError,
    StatusCode,
    UpdateTodoList,
    NewTodoList,
    FullTodoListInfo
};

pub async fn insert_todo_list(user_id: Uuid, todo_list: &NewTodoList, db_pool: &PgPool) -> Result<Uuid, ServiceError> {
    let id = uuid::Uuid::new_v4();

    sqlx::query!(
            "INSERT INTO todo_lists
            VALUES ($1, $2, $3)",
            id,
            user_id,
            todo_list.name
        ).execute(db_pool)
        .await
        .map_err(|e| ServiceError { status_code: StatusCode::InternalError, detail: Some(e.to_string())})?;

    Ok(id)
}

pub async fn select_todo_list_id(user_id: Uuid, db_pool: &PgPool) -> Result<Option<Uuid>, ServiceError> {
    let result = sqlx::query!(
            "SELECT id
            FROM todo_lists
            WHERE user_id = $1
            LIMIT 1",
            user_id
        )
        .fetch_all(db_pool)
        .await
        .map_err(|e| ServiceError { status_code: StatusCode::InternalError, detail: Some(e.to_string())})?;

        Ok(result.first().map(|r| r.id))
}

pub async fn delete_todo_list(user_id: Uuid, db_pool: &PgPool) -> Result<(), ServiceError> {
    sqlx::query!(
            "DELETE FROM todo_lists
            WHERE user_id = $1",
            user_id
        )
        .execute(db_pool)
        .await
        .map_err(|e| ServiceError { status_code: StatusCode::InternalError, detail: Some(e.to_string())})?;

    Ok(())
}

pub async fn update_todo_list(todo_list_id: Uuid, update_list: &UpdateTodoList, db_pool: &PgPool) -> Result<(), ServiceError> {
    sqlx::query!(
            "UPDATE todo_lists
            SET name = $1
            WHERE id = $2",
            update_list.name,
            todo_list_id
        )
        .execute(db_pool)
        .await
        .map_err(|e| ServiceError { status_code: StatusCode::InternalError, detail: Some(e.to_string())})?;

    Ok(())
}

pub async fn select_todo_list(user_id: Uuid, db_pool: &PgPool) -> Result<Option<FullTodoListInfo>, ServiceError> {
    let result = sqlx::query_as!(
            FullTodoListInfo,
            "SELECT id, user_id, name
            FROM todo_lists
            WHERE user_id = $1",
            user_id
        )
        .fetch_all(db_pool)
        .await
        .map_err(|e| ServiceError { status_code: StatusCode::InternalError, detail: Some(e.to_string())})?;

    Ok(result.into_iter().nth(0))
}