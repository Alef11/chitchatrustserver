use std::time::Duration;
use crate::db::db;
use crate::logger::logger::log_message;

pub async fn wait_for_db_connection() {
    let max_retries = 5;
    let delay = Duration::from_secs(5);

    for attempt in 1..=max_retries{
        if attempt == max_retries {
            panic!("Failed to connect to the database after {} attempts", max_retries);
        } else {
            if(db::DB_POOL.get_conn().is_ok()){
                log_message("Successfully connected to the database", file!());
                break;
            } else {
                log_message(&format!("Attempt {}: Database connection failed, retrying in {} seconds...", attempt, delay.as_secs()), file!());
                std::thread::sleep(delay);
            }
        }
    }
}