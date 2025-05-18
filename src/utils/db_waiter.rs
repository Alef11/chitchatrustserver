use std::time::Duration;
use crate::db::db;
use crate::log;

pub async fn wait_for_db_connection() {
    let max_retries = 5;
    let delay = Duration::from_secs(5);

    log!("Trying to reach the database...");

    for attempt in 1..=max_retries{
        if attempt == max_retries {
            panic!("Failed to connect to the database after {} attempts", max_retries);
        } else {
            if(db::DB_POOL.get_conn().is_ok()){
                log!("Successfully connected to the database");
                break;
            } else {
                log!(&format!("Attempt {}: Database connection failed, retrying in {} seconds...", attempt, delay.as_secs()));
                std::thread::sleep(delay);
            }
        }
    }
}