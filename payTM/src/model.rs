use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct SignupInput {
    pub username: String,
    pub password: String,
    pub firstname: String,
    pub lastname: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct SigninInput {
    pub username: String,
    pub password: String,
}

#[derive(Serialize, Debug)]
pub struct UserOut {
    pub id: i64,
    pub username: String,
    pub firstname: String,
    pub lastname: String,
}
