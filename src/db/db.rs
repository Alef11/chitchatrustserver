use mysql::prelude::*;
use mysql::*;
use std::sync::LazyLock;

use crate::modules::user::user;
use crate::utils::env_provider;
use crate::utils::xtime::Xtime;

static DB_CHARACTER_LIMIT: usize = 100;

// Replace with your actual MariaDB connection URL
pub static DB_POOL: LazyLock<Pool> = LazyLock::new(|| {
    Pool::new(env_provider::DATABASE_URL.as_str()).expect("Failed to create DB pool")
});

pub fn init_db() -> Result<()> {
    create_users_table()?;
    create_messages_table()?;
    create_groups_table()?;
    create_group_members_table()?;
    Ok(())
}

pub fn create_users_table() -> Result<()> {
    let mut conn = DB_POOL.get_conn()?;
    let query = format!(
        r"CREATE TABLE IF NOT EXISTS users (
            uuid int NOT NULL AUTO_INCREMENT PRIMARY KEY,
            username varchar({}) NOT NULL,
            password varchar({}) NOT NULL,
            email varchar({}),
            created_at DATETIME NOT NULL,
            last_online DATETIME NOT NULL,
            is_admin boolean NOT NULL DEFAULT false
        )",
        DB_CHARACTER_LIMIT, DB_CHARACTER_LIMIT, DB_CHARACTER_LIMIT
    );

    conn.query_drop(query)?;
    Ok(())
}

pub fn create_messages_table() -> Result<()> {
    let mut conn = DB_POOL.get_conn()?;
    let query = format!(
        r"CREATE TABLE IF NOT EXISTS messages (
            umid int NOT NULL AUTO_INCREMENT PRIMARY KEY,
            sender_id INT NOT NULL,
            recipient_id INT,
            group_id INT,
            content TEXT NOT NULL,
            sent_at DATETIME NOT NULL,
            FOREIGN KEY (sender_id) REFERENCES users(uuid),
            FOREIGN KEY (recipient_id) REFERENCES users(uuid),
            FOREIGN KEY (group_id) REFERENCES groups(id)
        )"
    );

    conn.query_drop(query)?;
    Ok(())
}

pub fn create_groups_table() -> Result<()> {
    let mut conn = DB_POOL.get_conn()?;
    let query = format!(
        r"CREATE TABLE IF NOT EXISTS groups (
            id int NOT NULL AUTO_INCREMENT PRIMARY KEY,
            name varchar({}) NOT NULL,
            owner_id INT NOT NULL,
            created_at DATETIME NOT NULL,
            FOREIGN KEY (owner_id) REFERENCES users(uuid)
        )",
        DB_CHARACTER_LIMIT
    );

    conn.query_drop(query)?;
    Ok(())
}

pub fn create_group_members_table() -> Result<()> {
    let mut conn = DB_POOL.get_conn()?;
    let query = format!(
        r"CREATE TABLE IF NOT EXISTS group_members (
            group_id INT NOT NULL,
            user_id INT NOT NULL,
            joined_at DATETIME NOT NULL,
            PRIMARY KEY (group_id, user_id),
            FOREIGN KEY (group_id) REFERENCES groups(id) on delete cascade,
            FOREIGN KEY (user_id) REFERENCES users(uuid) on delete cascade
        )"
    );

    conn.query_drop(query)?;
    Ok(())
}

pub fn create_token_table() -> Result<()> {
    let mut conn = DB_POOL.get_conn()?;
    let query = format!(
        r"CREATE TABLE IF NOT EXISTS tokens (
            user_id INT NOT NULL,
            token varchar(256) NOT NULL PRIMARY KEY,
            created_at DATETIME NOT NULL,
            expires_at DATETIME NOT NULL,
            FOREIGN KEY (user_id) REFERENCES users(uuid)
        )"
    );

    conn.query_drop(query)?;
    Ok(())
}

pub fn insert_user(user: &user) -> Result<u64> {
    let mut conn = DB_POOL.get_conn()?;

    let (query, bindings) = user.create_sql();

    conn.exec_drop(query, bindings)?;

    let uuid = conn.last_insert_id();

    Ok(uuid)
}

pub fn get_user_by_id(uuid: u64) -> Result<Option<user>> {
    let mut conn = DB_POOL.get_conn()?;

    let result: Option<Row> = conn.exec_first(
        "SELECT * FROM users WHERE uuid = :uuid",
        params! {
            "uuid" => uuid,
        },
    )?;

    Ok(result.map(user::from_row))
}

pub fn insert_token(user_id: u32, token: &str, experies_at: Xtime) -> Result<()> {
    let mut conn = DB_POOL.get_conn()?;

    conn.exec_drop(
        "INSERT INTO tokens (user_id, token, created_at, expires_at) VALUES (:user_id, :token, :created_at, :expires_at)",
        params! {
            "user_id" => user_id,
            "token" => token,
            "created_at" => "DATETIME(NOW())",
            "expires_at" => experies_at.to_mariadb_datetime(),
        },
    )?;

    Ok(())
}
