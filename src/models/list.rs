use serde::{
    Deserialize,
    Serialize
};
use uuid::Uuid;

#[derive(Deserialize)]
pub struct NewTodoList {
    pub name: String,
}

#[derive(Deserialize)]
pub struct UpdateTodoList {
    pub name: String,
}

#[derive(Serialize)]
pub struct FullTodoListInfo {
    pub id: Uuid,
    pub user_id: Uuid,
    pub name: String,
}