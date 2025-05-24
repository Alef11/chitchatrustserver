use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct LoginResponse {
    pub token: String,
    pub user_id: u32,
}

#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub error: String,
}

#[derive(Debug, Deserialize)]
pub struct TokenLoginRequest {
    pub uuid: u32,
    pub token: String,
}

#[derive(Debug, Deserialize)]
pub struct RegisterRequest {
    pub username: String,
    pub password: String,
    pub email: String,
}

#[derive(serde::Serialize)]
pub struct UserResponse {
    pub username: String,
    pub status: String,
}
