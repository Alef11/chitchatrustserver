use rocket::serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct LoginResponse {
    pub token: String,
    pub user_id: u32,
}

#[derive(Debug, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct ErrorResponse {
    pub error: String,
}

#[derive(Debug, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct TokenLoginRequest {
    pub uuid: u32,
    pub token: String,
}

#[derive(Debug, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct RegisterRequest {
    pub username: String,
    pub password: String,
    pub email: String,
}