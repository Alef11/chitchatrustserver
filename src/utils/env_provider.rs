use dotenv::dotenv;
use std::env;

lazy_static::lazy_static! {
    pub static ref MARIADB_ROOT_PASSWORD: String = env::var("MARIADB_ROOT_PASSWORD").expect("MARIADB_ROOT_PASSWORD must be set");
    pub static ref MARIADB_USER: String = env::var("MARIADB_USER").expect("MARIADB_USER must be set");
    pub static ref MARIADB_PASSWORD: String = env::var("MARIADB_PASSWORD").expect("MARIADB_PASSWORD must be set");
}

pub fn load_env() {
    // Load the environment variables from the .env file
    dotenv().ok();
}
