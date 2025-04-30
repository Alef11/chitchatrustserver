use chitchatrustserver::db::db::{DB_POOL};
use mysql::prelude::Queryable;

#[test]
fn test_db_pool_connection() {
    let mut conn = DB_POOL.get_conn()
        .expect("Failed to get connection from DB_POOL");

    // Simple test query
    let result: i32 = conn.query_first("SELECT 1")
        .expect("Query failed")
        .expect("No result returned");

    assert_eq!(result, 1);
}
