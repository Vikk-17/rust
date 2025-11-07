use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct UserInput {
    pub title: String,
    pub body: String,
}

