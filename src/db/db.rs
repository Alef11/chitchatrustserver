use std::sync::LazyLock;
use mysql::*;
use mysql::prelude::*;

// Replace with your actual MariaDB connection URL
pub static DB_POOL: LazyLock<Pool> = LazyLock::new(|| {
    let url = "mysql://user:password@localhost:3306/chitchat_db";
    Pool::new(url).expect("Failed to create DB pool")
});

pub fn create_users_table() -> Result<()> {
    let mut conn = DB_POOL.get_conn()?;
    conn.query_drop(
        r"CREATE TABLE IF NOT EXISTS users (
            uuid int NOT NULL AUTO_INCREMENT PRIMARY KEY,
            username varchar(255) NOT NULL,
            password varchar(255) NOT NULL,
            email varchar(255),
            created_at varchar(19) NOT NULL,
            last_online varchar(19) NOT NULL,
            is_admin boolean NOT NULL DEFAULT false,
        )"
    )?;
    Ok(())
}
