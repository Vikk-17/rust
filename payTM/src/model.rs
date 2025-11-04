use serde::{Deserialize, Serialize};
// use uuid::Uuid;

#[derive(Deserialize, Serialize, Debug)]
pub struct SingupInput {
    pub username: String,
    pub password: String,
    pub firstname: String,
    pub lastname: String,
}

#[derive(Serialize, Debug)]
pub struct UserOut {
    pub id: i64,
    pub username: String,
    pub firstname: String,
    pub lastname: String
}

