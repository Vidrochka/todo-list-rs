use actix_web::{
    HttpResponse,
    http::{
        self,
        header::ContentType
    },
    error
};
use derive_more::Display;
use serde::Serialize;

#[derive(Serialize, Debug, Display)]
pub enum StatusCode {
    #[serde(rename(serialize = "400 Bad Request"))] 
    BadRequest,
    #[serde(rename(serialize = "401 Unauthorized"))] 
    Unauthorized,
    #[serde(rename(serialize = "404 Not Found"))] 
    NotFound,
    #[serde(rename(serialize = "500 Internal Error"))] 
    InternalError,
}

#[derive(Serialize, Debug, Display)]
#[display(fmt = "{}", "serde_json::to_string(self).unwrap()")]
pub struct ServiceError {
    pub status_code: StatusCode,
    pub detail: Option<String>,
}

impl error::ResponseError for ServiceError {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::json())
            .body(serde_json::to_string(self).unwrap())
    }

    fn status_code(&self) -> http::StatusCode {
        match self.status_code {
            StatusCode::BadRequest => http::StatusCode::BAD_REQUEST,
            StatusCode::Unauthorized => http::StatusCode::UNAUTHORIZED,
            StatusCode::NotFound => http::StatusCode::NOT_FOUND,
            StatusCode::InternalError => http::StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

// impl ServiceError {
//     fn from_status_code(status_code: StatusCode) -> Self {
//         Self {
//             status_code,
//             detail: None,
//         }
//     }
// }