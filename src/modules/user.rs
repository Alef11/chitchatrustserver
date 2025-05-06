use mysql::{Params, Row, params};

use crate::utils::{
    encryption::{check_password, encrypt},
    xtime::Xtime,
};

pub struct User {
    pub uuid: u32,
    pub username: String,
    pub password: String,
    pub email: String,
    pub created_at: Xtime,
    pub last_online: Xtime,
    pub is_admin: bool,
}

impl User {
    pub fn new(username: String, password: String, email: String) -> Self {
        let encrypted_password = encrypt(&password);
        let generated_uuid = 0; //if uuid 0 user is not in db yet
        User {
            uuid: generated_uuid,
            username,
            password: encrypted_password,
            email,
            created_at: Xtime::now(),
            last_online: Xtime::now(),
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
                "created_at" => &self.created_at.to_mariadb_datetime(),
                "last_online" => &self.last_online.to_mariadb_datetime(),
                "is_admin" => self.is_admin,
            },
        )
    }

    pub fn from_row(row: Row) -> Self {
        let (year, month, day, hour, minute, second, _microsecond): (u16, u8, u8, u8, u8, u8, u32) =
            match row.get("created_at").unwrap() {
                mysql::Value::Date(y, m, d, h, min, s, micros) => (y, m, d, h, min, s, micros),
                _ => panic!("Invalid value for created_at"),
            };

        let created_at = Xtime::new(second, minute, hour, day, month, year);

        let (year2, month2, day2, hour2, minute2, second2, _): (u16, u8, u8, u8, u8, u8, u32) =
            match row.get("last_online").unwrap() {
                mysql::Value::Date(y, m, d, h, min, s, micros) => (y, m, d, h, min, s, micros),
                _ => panic!("Invalid value for last_online"),
            };

        let last_online = Xtime::new(second2, minute2, hour2, day2, month2, year2);

        User {
            uuid: row.get("uuid").unwrap(),
            username: row.get("username").unwrap(),
            password: row.get("password").unwrap(),
            email: row.get("email").unwrap(),
            created_at,
            last_online,
            is_admin: row.get("is_admin").unwrap(),
        }
    }

    pub fn to_string(&self) -> String {
        format!(
            "User {{ uuid: {}, username: {}, email: {}, created_at: {}, last_online: {}, is_admin: {} }}",
            self.uuid,
            self.username,
            self.email,
            self.created_at.to_string(),
            self.last_online.to_string(),
            self.is_admin
        )
    }

    pub fn get_uuid(&self) -> u32 {
        self.uuid
    }
}
