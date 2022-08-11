use sqlx::PgPool;
use uuid::Uuid;

use crate::models::{
    ServiceError,
    StatusCode,
    FullTaskInfo,
    TaskRange
};

pub async fn delete_tasks_by_list_id(todo_list_id: Uuid, db_pool: &PgPool) -> Result<i64, ServiceError> {
    let result = sqlx::query!(
            "WITH deleted AS (DELETE FROM tasks
            WHERE todo_list_id = $1 RETURNING *)

            SELECT count(*) as count FROM deleted",
            todo_list_id
        )
        .fetch_one(db_pool)
        .await
        .map_err(|e| ServiceError { status_code: StatusCode::InternalError, detail: Some(e.to_string())})?;

    Ok(result.count.unwrap())
}

pub async fn select_task_count(todo_list_id: Uuid, db_pool: &PgPool) -> Result<i64, ServiceError> {
    let result = sqlx::query!(
            "SELECT COUNT(*) as count
            FROM tasks
            WHERE todo_list_id = $1",
            todo_list_id
        )
        .fetch_one(db_pool)
        .await
        .map_err(|e| ServiceError { status_code: StatusCode::InternalError, detail: Some(e.to_string())})?;

    Ok(result.count.unwrap())
}

pub async fn insert_task_to_end(todo_list_id: Uuid, description: String, db_pool: &PgPool) -> Result<Uuid, ServiceError> {
    let id = uuid::Uuid::new_v4();

    let task_count = select_task_count(todo_list_id, db_pool).await?;
    let task_order = (task_count + 1) as i32;

    sqlx::query!(
            "INSERT INTO tasks
            VALUES ($1, $2, $3, $4)",
            id,
            todo_list_id,
            description,
            task_order
        ).execute(db_pool)
        .await
        .map_err(|e| ServiceError { status_code: StatusCode::InternalError, detail: Some(e.to_string())})?;

    Ok(id)
}

pub async fn select_task(todo_list_id: Uuid, task_id: Uuid, db_pool: &PgPool) -> Result<Option<FullTaskInfo>, ServiceError> {
    let result = sqlx::query_as!(
            FullTaskInfo,
            "SELECT id, todo_list_id, description, \"order\"
            FROM tasks
            WHERE todo_list_id = $1 AND id = $2",
            todo_list_id,
            task_id
        )
        .fetch_optional(db_pool)
        .await
        .map_err(|e| ServiceError { status_code: StatusCode::InternalError, detail: Some(e.to_string())})?;

    Ok(result)
}

pub async fn insert_task(todo_list_id: Uuid, description: String, order: i32, db_pool: &PgPool) -> Result<Uuid, ServiceError> {
    let id = uuid::Uuid::new_v4();

    offset_add_or_remove_space(todo_list_id, order, 1, db_pool).await?;

    sqlx::query!(
            "INSERT INTO tasks
            VALUES ($1, $2, $3, $4);",
            id,
            todo_list_id,
            description,
            order
        ).execute(db_pool)
        .await
        .map_err(|e| ServiceError { status_code: StatusCode::InternalError, detail: Some(e.to_string())})?;

    Ok(id)
}

pub async fn select_tasks(todo_list_id: Uuid, db_pool: &PgPool) -> Result<Vec<FullTaskInfo>, ServiceError> {
    let result = sqlx::query_as!(
            FullTaskInfo,
            "SELECT id, todo_list_id, description, \"order\"
            FROM tasks
            WHERE todo_list_id = $1",
            todo_list_id
        )
        .fetch_all(db_pool)
        .await
        .map_err(|e| ServiceError { status_code: StatusCode::InternalError, detail: Some(e.to_string())})?;

    Ok(result)
}

pub async fn select_tasks_range(todo_list_id: Uuid, range: TaskRange, db_pool: &PgPool) -> Result<Vec<FullTaskInfo>, ServiceError> {
    let result = sqlx::query_as!(
            FullTaskInfo,
            "SELECT id, todo_list_id, description, \"order\"
            FROM tasks
            WHERE todo_list_id = $1
            ORDER BY \"order\"
            LIMIT $2 OFFSET $3",
            todo_list_id,
            range.count as i64,
            range.offset as i64
        )
        .fetch_all(db_pool)
        .await
        .map_err(|e| ServiceError { status_code: StatusCode::InternalError, detail: Some(e.to_string())})?;

    Ok(result)
}

pub async fn delete_task(todo_list_id: Uuid, task_id: Uuid, db_pool: &PgPool) -> Result<Option<FullTaskInfo>, ServiceError> {
    let result = sqlx::query_as!(
            FullTaskInfo,
            "WITH deleted AS (DELETE FROM tasks
            WHERE todo_list_id = $1 AND id = $2 RETURNING *)

            SELECT * FROM deleted",
            todo_list_id,
            task_id
        )
        .fetch_optional(db_pool)
        .await
        .map_err(|e| ServiceError { status_code: StatusCode::InternalError, detail: Some(e.to_string())})?;

    if let Some(task) = &result {
        offset_add_or_remove_space(todo_list_id, task.order, -1, db_pool).await?;
    }
    
    Ok(result)
}

pub async fn update_task(todo_list_id: Uuid, task_id: Uuid, description: String, db_pool: &PgPool) -> Result<Option<FullTaskInfo>, ServiceError> {
    let result = sqlx::query_as!(
            FullTaskInfo,
            "WITH update AS (UPDATE tasks
            SET description = $1
            WHERE todo_list_id = $2 AND id = $3 RETURNING *)
            SELECT * FROM update",
            description,
            todo_list_id,
            task_id
        ).fetch_optional(db_pool)
        .await
        .map_err(|e| ServiceError { status_code: StatusCode::InternalError, detail: Some(e.to_string())})?;

    Ok(result)
}

pub async fn move_task(todo_list_id: Uuid, task_id: Uuid, old_order: i32, new_order: i32, db_pool: &PgPool) -> Result<FullTaskInfo, ServiceError> {
    assert!(old_order != new_order);

    // if move item from right to left, then move range from left to right
    let range_move_left_to_right = old_order > new_order;

    let mut offset_bottom = std::cmp::min(old_order, new_order);
    let mut offset_top = std::cmp::max(old_order, new_order);
    let offset = if range_move_left_to_right { 1 } else { -1 };

    if range_move_left_to_right {
        offset_top = offset_top - 1;
    } else {
        offset_bottom = offset_bottom + 1;
    }

    offset_range(todo_list_id, offset_bottom, offset_top, offset, db_pool).await?;

    let result = sqlx::query_as!(
            FullTaskInfo,
            "WITH update AS (UPDATE tasks
            SET \"order\" = $1
            WHERE todo_list_id = $2 AND id = $3 RETURNING *)
            SELECT * FROM update",
            new_order,
            todo_list_id,
            task_id
        ).fetch_one(db_pool)
        .await
        .map_err(|e| ServiceError { status_code: StatusCode::InternalError, detail: Some(e.to_string())})?;

    Ok(result)
}

pub async fn offset_add_or_remove_space(todo_list_id: Uuid, order: i32, offset: i32, db_pool: &PgPool) -> Result<(), ServiceError> {
    sqlx::query!(
            "UPDATE tasks
            SET \"order\" = \"order\" + $1
            WHERE todo_list_id = $2 AND \"order\" >= $3;",
            offset,
            todo_list_id,
            order
        )
        .execute(db_pool)
        .await
        .map_err(|e| ServiceError { status_code: StatusCode::InternalError, detail: Some(e.to_string())})?;

    Ok(())
}

pub async fn offset_range(todo_list_id: Uuid, bottom: i32, top: i32, offset: i32, db_pool: &PgPool) -> Result<(), ServiceError> {
    sqlx::query!(
            "UPDATE tasks
            SET \"order\" = \"order\" + $1
            WHERE todo_list_id = $2 AND \"order\" >= $3 AND \"order\" <= $4;",
            offset,
            todo_list_id,
            bottom,
            top
        )
        .execute(db_pool)
        .await
        .map_err(|e| ServiceError { status_code: StatusCode::InternalError, detail: Some(e.to_string())})?;

    Ok(())
}

