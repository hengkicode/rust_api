use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Serialize, FromRow, Debug)]
pub struct User {
    pub id: i64,
    pub name: String,
    pub email: String,
}

#[derive(Deserialize, Debug)]  // Menambahkan derive Debug
pub struct CreateUserRequest {
    pub name: String,
    pub email: String,
}

#[derive(Deserialize, Debug)]  // Menambahkan derive Debug
pub struct UpdateUserRequest {
    pub name: Option<String>,
    pub email: Option<String>,
}
