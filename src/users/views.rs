use serde::Deserialize;

#[derive(Deserialize)]
pub struct UserForm {
    pub first_name: String,
    pub last_name: String,
    pub occupation: String
}
