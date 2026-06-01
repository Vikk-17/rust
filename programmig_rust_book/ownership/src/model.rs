use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Person {
    pub id: u32,
    pub name: String,
    pub age: u32,
    pub email: String,
    pub phone: String,
    pub city: String,
    pub country: String,
    pub occupation: String,
    pub salary: u64,
    pub is_active: bool,
}


#[derive(Debug, Deserialize)]
pub struct People {
    pub people: Vec<Person>,
}
