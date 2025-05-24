use chitchatrustserver::{
    db::db::{self, DB_POOL},
    modules::user::User,
    utils::encryption,
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
    db::create_users_table().expect("Failed to create users table");

    // Insert a test user
    let username = "testuser";
    let password = "testpassword";
    let email = "a@a.com";

    let user1 = User::new(
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

#[test]
fn test_user_exists() {
    db::create_users_table().expect("Failed to create users table");

    // Insert a test user
    let username = "testuserexist";
    let password = "testpassword";
    let email = "";

    let user1 = User::new(
        username.to_string(),
        password.to_string(),
        email.to_string(),
    );

    let _user_id = db::insert_user(&user1).expect("Failed to insert user");

    // Verify the user exists
    let exists = db::check_if_user_exists(username).expect("Failed to check if user exists");
    assert!(exists, "User should exist");
}

#[test]
fn test_start_prefix() {
    db::create_users_table().expect("Failed to create users table");

    // Insert a test user
    let user1 = User::new(
        "testuserprefix1".to_string(),
        "testpassword".to_string(),
        "a1@a.com".to_string(),
    );
    let user2 = User::new(
        "testuserprefix2".to_string(),
        "testpassword".to_string(),
        "a2@a.com".to_string(),
    );
    let user3 = User::new(
        "testuserprefix3".to_string(),
        "testpassword".to_string(),
        "a3@a.com".to_string(),
    );
    let user4 = User::new(
        "testuserprefix4".to_string(),
        "testpassword".to_string(),
        "a4@a.com".to_string(),
    );

    let _user_id1 = db::insert_user(&user1).expect("Failed to insert user");
    let _user_id2 = db::insert_user(&user2).expect("Failed to insert user");
    let _user_id3 = db::insert_user(&user3).expect("Failed to insert user");
    let _user_id4 = db::insert_user(&user4).expect("Failed to insert user");

    let resutl1: Vec<User> = db::get_users_starting_with("t").expect("Failed to get users");

    let expected_usernames = vec![
        user1.username.clone(),
        user2.username.clone(),
        user3.username.clone(),
        user4.username.clone(),
    ];
    let result_usernames: Vec<String> = resutl1.iter().map(|u| u.username.clone()).collect();

    for username in &expected_usernames {
        assert!(
            result_usernames.contains(username),
            "Username '{}' not found in result",
            username
        );
    }

    let result2: Vec<User> =
        db::get_users_starting_with("testuserprefix1").expect("Failed to get users");
    let result2_usernames: Vec<String> = result2.iter().map(|u| u.username.clone()).collect();

    // Assert result2 contains user1
    assert!(
        result2_usernames.contains(&user1.username),
        "result2 should contain user1"
    );

    // Assert result2 does NOT contain user2, user3, or user4
    for user in [&user2, &user3, &user4] {
        assert!(
            !result2_usernames.contains(&user.username),
            "result2 should NOT contain {}",
            user.username
        );
    }
}

#[test]
fn test_get_user_by_id() {
    db::create_users_table().expect("Failed to create users table");

    // Insert a test user
    let username = "testuserbyid";
    let password = "testpassword";
    let email = "a@a.com";
    let user1 = User::new(
        username.to_string(),
        password.to_string(),
        email.to_string(),
    );
    let user_id = db::insert_user(&user1).expect("Failed to insert user");
    let result = db::get_user_by_id(user_id)
        .expect("Failed to get user by ID")
        .expect("User not found");
    print!("User found: {:?}", result.to_string());
    assert_eq!(result.username, username);
}

#[test]
fn test_login_logic() {
    db::create_users_table().expect("Failed to create users table");

    let username = "testuserbyid";
    let password = "testpassword";
    let email = "a@a.com";
    let user1 = User::new(
        username.to_string(),
        password.to_string(),
        email.to_string(),
    );

    let user_id = db::insert_user(&user1).expect("Failed to insert user");
    let user = db::get_user_by_id(user_id)
        .expect("Failed to get user by ID")
        .expect("User not found");

    let result = encryption::check_password(password, &user.password);
    assert!(result, "Password check failed for user: {}", username);
}

#[test]
fn test_get_passowrd_username() {
    db::create_users_table().expect("Failed to create users table");

    let username = "testuserpasswordget";
    let password = "testpasswordget";
    let email = "a@a.com";

    let user1 = User::new(
        username.to_string(),
        password.to_string(),
        email.to_string(),
    );

    let _ = db::insert_user(&user1).expect("Failed to insert user");

    let pass = db::get_password_username(username).expect("Failed to get password for user");

    assert_eq!(
        encryption::encrypt(password),
        pass,
        "Password does not match for user: {}",
        username
    );

    assert_ne!(
        pass, "a",
        "Password should not be 'a' for user: {}",
        username
    );
}
