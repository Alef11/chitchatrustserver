use mysql::prelude::*;
use mysql::*;
use std::sync::LazyLock;

use crate::modules::user::user;

static DB_CHARACTER_LIMIT: usize = 100;

// Replace with your actual MariaDB connection URL
pub static DB_POOL: LazyLock<Pool> = LazyLock::new(|| {
    let url = "mysql://user:password@localhost:3306/chitchat_db";
    Pool::new(url).expect("Failed to create DB pool")
});

pub fn create_users_table() -> Result<()> {
    let mut conn = DB_POOL.get_conn()?;
    let query = format!(
        r"CREATE TABLE IF NOT EXISTS users (
            uuid int NOT NULL AUTO_INCREMENT PRIMARY KEY,
            username varchar({}) NOT NULL,
            password varchar({}) NOT NULL,
            email varchar({}),
            created_at varchar(19) NOT NULL,
            last_online varchar(19) NOT NULL,
            is_admin boolean NOT NULL DEFAULT false
        )",
        DB_CHARACTER_LIMIT, DB_CHARACTER_LIMIT, DB_CHARACTER_LIMIT
    );

    conn.query_drop(query)?;
    Ok(())
}

pub fn insert_user(user: &user) -> Result<()> {
    let mut conn = DB_POOL.get_conn()?;

    let (query, bindings) = user.create_sql();

    conn.exec_drop(query, bindings)?;

    Ok(())
}
