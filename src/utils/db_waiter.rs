use std::time::Duration;
use mysql::Pool;
use tokio::time::sleep;

use crate::log;

use super::env_provider;


pub async fn ensure_db_connection_ready() {
    let max_retries = 12;
    let delay = Duration::from_secs(5);

    let db_url = env_provider::DATABASE_URL.as_str();

    for attempt in 1..=max_retries {
        match Pool::new(db_url) {
            Ok(pool) => match pool.get_conn() {
                Ok(_) => {
                    log!("Successfully connected to the database");
                    return;
                }
                Err(e) => {
                    log!(&format!("Pool created, but failed to get connection: {}", e));
                }
            },
            Err(e) => {
                log!(&format!(
                    "Attempt {}: Failed to create pool: {}",
                    attempt, e
                ));
            }
        }

        if attempt == max_retries {
            panic!("Could not connect to DB after {} attempts", max_retries);
        }

        log!("Retrying in {} seconds...", delay.as_secs());
        sleep(delay).await;
    }
}