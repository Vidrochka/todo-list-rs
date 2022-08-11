use actix_web::{
    web,
    Result
};
use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    models::*,
    middlewares::BearerAuth,
    db::{
        list,
        task
    }
};

pub async fn new_task(new_task_info: web::Json<NewTask>, db_pool: web::Data<PgPool>, bearer_auth: BearerAuth) -> Result<String, ServiceError> {
    let todo_list_id = list::select_todo_list_id(bearer_auth.user_id, &**db_pool).await?
        .ok_or(ServiceError { status_code: StatusCode::NotFound, detail: Some(format!("TO-DO list not found")) })?;

    let id = match new_task_info.position {
        crate::models::TaskPosition::End => {
            task::insert_task_to_end(todo_list_id, new_task_info.description.clone(), &**db_pool).await?
        },
        crate::models::TaskPosition::After { task_id } => {
            let task = task::select_task(todo_list_id, task_id, &**db_pool).await?
                .ok_or(ServiceError { status_code: StatusCode::NotFound, detail: Some(format!("Task not found")) })?;

                task::insert_task(todo_list_id, new_task_info.description.clone(), task.order + 1, &**db_pool).await?
        },
        crate::models::TaskPosition::Before { task_id } => {
            let task = task::select_task(todo_list_id, task_id, &**db_pool).await?
                .ok_or(ServiceError { status_code: StatusCode::NotFound, detail: Some(format!("Task not found")) })?;

            // if task.order is min, then min - 1 less then min
            let order = std::cmp::max(task.order - 1, 1);
            
            task::insert_task(todo_list_id, new_task_info.description.clone(), order, &**db_pool).await?
        },
    };
        
    Ok(id.to_string())
}

pub async fn get_tasks(db_pool: web::Data<PgPool>, bearer_auth: BearerAuth) -> Result<web::Json<Vec<FullTaskInfo>>, ServiceError> {
    let todo_list_id = list::select_todo_list_id(bearer_auth.user_id, &**db_pool).await?
        .ok_or(ServiceError { status_code: StatusCode::NotFound, detail: Some(format!("TO-DO list not found")) })?;

    let mut tasks = task::select_tasks(todo_list_id, &**db_pool).await?;
    tasks.sort_by_key(|x| x.order);
        
    Ok(web::Json(tasks))
}

pub async fn get_tasks_range(range: web::Query<TaskRange>, db_pool: web::Data<PgPool>, bearer_auth: BearerAuth) -> Result<web::Json<Vec<FullTaskInfo>>, ServiceError> {
    let todo_list_id = list::select_todo_list_id(bearer_auth.user_id, &**db_pool).await?
        .ok_or(ServiceError { status_code: StatusCode::NotFound, detail: Some(format!("TO-DO list not found")) })?;

    let mut tasks = task::select_tasks_range(todo_list_id, range.into_inner(), &**db_pool).await?;
    tasks.sort_by_key(|x| x.order);
        
    Ok(web::Json(tasks))
}

pub async fn delete_tasks(task_id: web::Path<Uuid>, db_pool: web::Data<PgPool>, bearer_auth: BearerAuth) -> Result<web::Json<FullTaskInfo>, ServiceError> {
    let todo_list_id = list::select_todo_list_id(bearer_auth.user_id, &**db_pool).await?
        .ok_or(ServiceError { status_code: StatusCode::NotFound, detail: Some(format!("TO-DO list not found")) })?;

    let task_id = task_id.into_inner();

    let task = task::delete_task(todo_list_id, task_id, &**db_pool).await?
        .ok_or(ServiceError { status_code: StatusCode::NotFound, detail: Some(format!("Task not found")) })?;

    Ok(web::Json(task))
}

pub async fn update_task(task_id: web::Path<Uuid>, new_task_info: web::Json<UpdateTask>, db_pool: web::Data<PgPool>, bearer_auth: BearerAuth) -> Result<web::Json<FullTaskInfo>, ServiceError> {
    let todo_list_id = list::select_todo_list_id(bearer_auth.user_id, &**db_pool).await?
        .ok_or(ServiceError { status_code: StatusCode::NotFound, detail: Some(format!("TO-DO list not found")) })?;

    let task_id = task_id.into_inner();

    let task = task::update_task(todo_list_id, task_id, new_task_info.description.clone(), &**db_pool).await?
        .ok_or(ServiceError { status_code: StatusCode::NotFound, detail: Some(format!("Task not found")) })?;

    Ok(web::Json(task))
}

pub async fn move_task(task_id: web::Path<Uuid>, new_task_info: web::Json<MoveTask>, db_pool: web::Data<PgPool>, bearer_auth: BearerAuth) -> Result<web::Json<FullTaskInfo>, ServiceError> {
    let todo_list_id = list::select_todo_list_id(bearer_auth.user_id, &**db_pool).await?
        .ok_or(ServiceError { status_code: StatusCode::NotFound, detail: Some(format!("TO-DO list not found")) })?;

    let id = task_id.into_inner();

    let source_position_order = task::select_task(todo_list_id, id, &**db_pool).await?
        .ok_or(ServiceError { status_code: StatusCode::NotFound, detail: Some(format!("Task not found")) })?
        .order;

    let task = match new_task_info.position {
        crate::models::TaskPosition::End => {
            let task_count = task::select_task_count(todo_list_id, &**db_pool).await?;
            let destination_position_order = task_count as i32;

            task::move_task(todo_list_id, id, source_position_order, destination_position_order, &**db_pool).await?
        },
        crate::models::TaskPosition::After { task_id } => {
            if id == task_id {
                return Err(ServiceError { status_code: StatusCode::BadRequest, detail: Some(format!("Source and destination task id is the same")) });
            }

            let task = task::select_task(todo_list_id, task_id, &**db_pool).await?
                .ok_or(ServiceError { status_code: StatusCode::NotFound, detail: Some(format!("Task not found")) })?;

            // 1, 2, 3, 4, 5 , 6 if move 6 after 2, order is 3, if move 2 after 6 order is 6, because range 3-6 move -1
            let destination_position_order = if source_position_order < task.order { task.order } else { task.order + 1};

            task::move_task(todo_list_id, id, source_position_order, destination_position_order, &**db_pool).await?
        },
        crate::models::TaskPosition::Before { task_id } => {
            if id == task_id {
                return Err(ServiceError { status_code: StatusCode::BadRequest, detail: Some(format!("Source and destination task id is the same")) });
            }

            let task = task::select_task(todo_list_id, task_id, &**db_pool).await?
                .ok_or(ServiceError { status_code: StatusCode::NotFound, detail: Some(format!("Task not found")) })?;

            // 1, 2, 3, 4, 5 , 6 if move 6 before 2, order is 2, if move 2 before 6 order is 6, because range 3-6 move -1
            let destination_position_order = if source_position_order < task.order { task.order - 1 } else { task.order };

            // if task.order is min, then min - 1 less then min
            let destination_position_order = std::cmp::max(destination_position_order, 1);
            
            task::move_task(todo_list_id, id, source_position_order, destination_position_order, &**db_pool).await?
        },
    };

    Ok(web::Json(task))
}
