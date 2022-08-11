use serde::Deserialize;

#[derive(Deserialize)]
pub struct NewUser {
    pub login: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct Login {
    pub login: String,
    pub password: String,
}