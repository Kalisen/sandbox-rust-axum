use serde::Serialize;

#[derive(Clone, Serialize)]
pub struct User {
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
    pub occupation: String
}