
use std::env;

use crate::models::*;

use actix_web::{
    dev,
    FromRequest,
    HttpRequest
};
use futures::future::{
    err,
    ok,
    Ready
};
use jsonwebtoken::{
    decode,
    Algorithm,
    DecodingKey,
    Validation
};
use uuid::Uuid;

pub struct BearerAuth {
    pub user_id: Uuid,
}

const BEARER_KEY_ENV: &'static str = "BEARER_KEY";

impl FromRequest for BearerAuth {
    type Error = ServiceError;
    type Future = Ready<Result<BearerAuth, ServiceError>>;

    fn from_request(req: &HttpRequest, _payload: &mut dev::Payload) -> Self::Future {
        let auth = req.headers().get("Authorization");
        match auth {
            Some(_) => {
                let _split: Vec<&str> = auth.unwrap().to_str().unwrap().split("Bearer").collect();
                let token = _split[1].trim();

                let bearer_key = env::var(BEARER_KEY_ENV)
                    .expect(&*format!("Env {BEARER_KEY_ENV} not found"));

                match decode::<UserClaim>(
                    token,
                    &DecodingKey::from_secret(bearer_key.as_bytes()),
                    &Validation::new(Algorithm::HS256),
                ) {
                    Ok(claims) => ok(BearerAuth { user_id: claims.claims.user_id }),
                    Err(_e) => err(ServiceError { status_code: StatusCode::Unauthorized, detail: Some("invalid token!".to_string())}),
                }
            }
            None => err(ServiceError { status_code: StatusCode::BadRequest, detail: Some("Authorization header not found".to_string())}),
        }
    }
}