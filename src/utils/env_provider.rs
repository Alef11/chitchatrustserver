use dotenv::dotenv;
use std::sync::Once;
use crate::logger::logger::LogExpect;

static INIT: Once = Once::new();

fn init_env() {
    INIT.call_once(|| {
        dotenv().ok();
    });
}

/// Macro to safely read env vars with fallback or panic
#[macro_export]
macro_rules! env_var {
    ($key:expr) => {{
        init_env();
        std::env::var($key).log_expect(concat!($key, " must be set"), file!())
    }};
    ($key:expr, $default:expr) => {{
        init_env();
        std::env::var($key).unwrap_or_else(|_| $default.to_string())
    }};
}

lazy_static::lazy_static! {
    pub static ref MARIADB_USER: String = env_var!("MARIADB_USER");
    pub static ref MARIADB_PASSWORD: String = env_var!("MARIADB_PASSWORD");
    pub static ref MARIADB_HOST: String = env_var!("MARIADB_HOST", "localhost");
    pub static ref MARIADB_PORT: String = env_var!("MARIADB_PORT", "3306");
    pub static ref MARIADB_DB: String = env_var!("MARIADB_DATABASE");
    pub static ref DATABASE_URL: String = format!(
        "mysql://{}:{}@{}:{}/{}",
        *MARIADB_USER,
        *MARIADB_PASSWORD,
        *MARIADB_HOST,
        *MARIADB_PORT,
        *MARIADB_DB
    );
    pub static ref PUBLIC_DOMAIN: String = env_var!("PUBLIC_DOMAIN", "localhost");
}
