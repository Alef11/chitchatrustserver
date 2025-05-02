use mysql::{Params, params};

use crate::utils::{
    encryption::{check_password, encrypt},
    xtime::Xtime,
};

pub struct user {
    pub uuid: i32,
    pub username: String,
    pub password: String,
    pub email: String,
    pub created_at: String,
    pub last_online: String,
    pub is_admin: bool,
}

impl user {
    pub fn new(username: String, password: String, email: String) -> Self {
        let encrypted_password = encrypt(&password);
        let generated_uuid = 0; //TODO: generate a unique UUID for the user
        user {
            uuid: generated_uuid,
            username,
            password: encrypted_password,
            email,
            created_at: Xtime::now().to_string(),
            last_online: Xtime::now().to_string(),
            is_admin: false,
        }
    }

    pub fn verify_password(&self, password: &str) -> bool {
        check_password(password, &self.password)
    }

    pub fn create_sql(&self) -> (&'static str, Params) {
        (
            "INSERT INTO users (username, password, email, created_at, last_online, is_admin)
             VALUES (:username, :password, :email, :created_at, :last_online, :is_admin)",
            params! {
                "username" => &self.username,
                "password" => &self.password,
                "email" => &self.email,
                "created_at" => &self.created_at,
                "last_online" => &self.last_online,
                "is_admin" => self.is_admin,
            },
        )
    }

    pub fn get_uuid(&self) -> i32 {
        self.uuid
    }
}
