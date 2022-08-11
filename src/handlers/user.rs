use std::env;

use actix_web::{
    web,
    Result
};
use chrono::{
    Utc,
    Duration
};
use jsonwebtoken::{
    EncodingKey,
    encode
};
use sqlx::PgPool;

use crate::{
    models::*,
    db::user
};

pub async fn register(db_pool: web::Data<PgPool>, new_user_info: web::Json<NewUser>) -> Result<String, ServiceError> {
    if user::is_user_exist(&new_user_info.login, &**db_pool).await? {
        return Err(ServiceError { status_code: StatusCode::BadRequest, detail: Some(format!("User with login name \"{}\" already exists", new_user_info.login)) })
    }

    let id = user::insert_user(&*new_user_info, &**db_pool).await?;
        
    Ok(id.to_string())
}

const BEARER_KEY_ENV: &'static str = "BEARER_KEY";

pub async fn login(db_pool: web::Data<PgPool>, login_info: web::Json<Login>) -> Result<String, ServiceError> {
    let user_id = user::select_user_id(&login_info.login, &login_info.password, &**db_pool).await?
        .ok_or(ServiceError { status_code: StatusCode::BadRequest, detail: Some(format!("User with login name \"{}\" not found", login_info.login)) })?;
    
    let date = Utc::now() + Duration::hours(1);

    let user_claim = UserClaim {
        exp: date.timestamp() as usize,
        user_id: user_id,
    };

    let bearer_key = env::var(BEARER_KEY_ENV)
        .expect(&*format!("Env {BEARER_KEY_ENV} not found"));

    let token = encode(
            &jsonwebtoken::Header::default(),
            &user_claim,
            &EncodingKey::from_secret(bearer_key.as_bytes()),
        )
        .map_err(|e| ServiceError { status_code: StatusCode::InternalError, detail: Some(e.to_string()) })?;

    Ok(token)
}