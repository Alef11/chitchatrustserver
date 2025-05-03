use chitchatrustserver::{
    db::db::{self, DB_POOL},
    modules::user::user,
    utils::xtime::Xtime,
};
use mysql::prelude::Queryable;

#[test]
fn test_db_pool_connection() {
    let mut conn = DB_POOL
        .get_conn()
        .expect("Failed to get connection from DB_POOL");

    // Simple test query
    let result: i32 = conn
        .query_first("SELECT 1")
        .expect("Query failed")
        .expect("No result returned");

    assert_eq!(result, 1);
}

#[test]
fn test_insert_user() {
    let create_table = db::create_users_table().expect("Failed to create users table");

    let mut conn = DB_POOL
        .get_conn()
        .expect("Failed to get connection from DB_POOL");

    // Insert a test user
    let username = "testuser";
    let password = "testpassword";
    let email = "a@a.com";

    let user1 = user::new(
        username.to_string(),
        password.to_string(),
        email.to_string(),
    );

    let user_id = db::insert_user(&user1).expect("Failed to insert user");

    // Verify the user was inserted correctly
    let result = db::get_user_by_id(user_id)
        .expect("Failed to get user by ID")
        .expect("User not found");

    print!("User found: {:?}", result.to_string());

    assert_eq!(result.username, username);
}
