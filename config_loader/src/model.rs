use serde::{Serialize, Deserialize};
use std::fs::File;

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub _id: String,
    pub index: u8,
    pub guid: String,
    pub isActive: bool,
    pub balance: String,
    pub picture: String,
    pub age: u8,
    pub eyeColor: String,
    pub name: String,
    pub gender: String,
    pub company: String,
    pub email: String,
    pub phone: String,
    pub address: String,
    pub about: String,
    pub registered: String,
    pub latitude: f32,
    pub longitude: f32,
    pub tags: Vec<String>,
    pub greeting: String,
    pub favoriteFruit: String,
    pub friends: Vec<Friend>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Friend {
    pub id: u32,
    pub name: String,
}

pub fn read_json_file(path: &str) -> Result<Config, Box<dyn std::error::Error>> {
    let file = File::open(path).unwrap();
    let data: Config = serde_json::from_reader(file).unwrap();

    Ok(data)
}

