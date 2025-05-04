use dotenv::dotenv;
use std::{env, sync::Once};

static INIT: Once = Once::new();

fn init_env() {
    INIT.call_once(|| {
        dotenv().ok();
    });
}

lazy_static::lazy_static! {
    pub static ref MARIADB_USER: String = {
        init_env();
        env::var("MARIADB_USER").expect("MARIADB_USER must be set")
    };
    pub static ref MARIADB_PASSWORD: String = {
        init_env();
        env::var("MARIADB_PASSWORD").expect("MARIADB_PASSWORD must be set")
    };
    pub static ref DATABASE_URL: String = format!(
        "mysql://{}:{}@localhost:3306/chitchat_db",
        *MARIADB_USER,
        *MARIADB_PASSWORD
    );
}
