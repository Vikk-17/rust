use std::sync::Mutex;

#[allow(unused)]
pub struct AppState {
    pub app_name: String,
    pub author: String,
    pub year: u16,
}

pub struct AppStateWithMutex {
    pub counter: Mutex<i32>,
}

