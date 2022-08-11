use actix_web::{web, Result};
use sqlx::PgPool;

use crate::{
    models::{
        ServiceError,
        StatusCode,
        NewTodoList,
        UpdateTodoList,
        FullTodoListInfo
    },
    middlewares::BearerAuth,
    db::{
        list::{
            select_todo_list_id,
            delete_todo_list,
            update_todo_list,
            insert_todo_list,
            select_todo_list
        },
        task::delete_tasks_by_list_id
    }
};

pub async fn new_list(db_pool: web::Data<PgPool>, new_list_info: web::Json<NewTodoList>, bearer_auth: BearerAuth) -> Result<String, ServiceError> {
    if let Some(todo_list_id) = select_todo_list_id(bearer_auth.user_id, &**db_pool).await? {
        return Err(ServiceError { status_code: StatusCode::BadRequest, detail: Some(format!("You have already TO-DO list with id = {todo_list_id}")) })
    }

    let id = insert_todo_list(bearer_auth.user_id, &*new_list_info, &**db_pool).await?;
        
    Ok(id.to_string())
}

pub async fn delete_list(db_pool: web::Data<PgPool>, bearer_auth: BearerAuth) -> Result<String, ServiceError> {
    let todo_list_id = select_todo_list_id(bearer_auth.user_id, &**db_pool).await?
        .ok_or(ServiceError { status_code: StatusCode::NotFound, detail: Some(format!("TO-DO list not found")) })?;

    let count = delete_tasks_by_list_id(todo_list_id, &*db_pool).await?;
    delete_todo_list(bearer_auth.user_id, &*db_pool).await?;
        
    Ok(count.to_string())
}

pub async fn update_list(db_pool: web::Data<PgPool>, list_info: web::Json<UpdateTodoList>, bearer_auth: BearerAuth) -> Result<String, ServiceError> {
    let todo_list_id = select_todo_list_id(bearer_auth.user_id, &**db_pool).await?
        .ok_or(ServiceError { status_code: StatusCode::NotFound, detail: Some(format!("TO-DO list not found")) })?;

    update_todo_list(todo_list_id, &*list_info, &**db_pool).await?;
        
    Ok(todo_list_id.to_string())
}

pub async fn get_list(db_pool: web::Data<PgPool>, bearer_auth: BearerAuth) -> Result<web::Json<FullTodoListInfo>, ServiceError> {
    let todo_list = select_todo_list(bearer_auth.user_id, &**db_pool).await?
        .ok_or(ServiceError { status_code: StatusCode::NotFound, detail: Some(format!("TO-DO list not found")) })?;
        
    Ok(web::Json(todo_list))
}