use serde::{Serialize, Deserialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct UserClaim {
    pub exp: usize,
    pub user_id: Uuid,
}