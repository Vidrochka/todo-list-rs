use serde::{
    Deserialize,
    Serialize
};
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
pub enum TaskPosition {
    #[serde(rename(deserialize = "end", serialize = "end"))] 
    End,
    #[serde(rename(deserialize = "after", serialize = "after"))] 
    After { task_id: Uuid },
    #[serde(rename(deserialize = "before", serialize = "before"))] 
    Before { task_id: Uuid },
}

#[derive(Deserialize)]
pub struct NewTask {
    pub description: String,
    pub position: TaskPosition,
}

#[derive(Serialize)]
pub struct FullTaskInfo {
    pub id: Uuid,
    pub todo_list_id: Uuid,
    pub description: String,
    pub order: i32,
}

#[derive(Deserialize)]
pub struct UpdateTask {
    pub description: String,
}

#[derive(Deserialize)]
pub struct MoveTask {
    pub position: TaskPosition,
}

#[derive(Deserialize)]
pub struct TaskRange {
    pub offset: u32,
    pub count: u32
}