use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    pub username: String,
    pub password: String,
    pub firstname: String,
    pub lastname: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct SingupInput {
    username: String,
    password: String,
    firstname: String,
    lastname: String,
}

