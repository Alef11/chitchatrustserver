use chitchatrustserver::db::db;
use chitchatrustserver::modules::user::User;
use chitchatrustserver::utils::token;
use chitchatrustserver::utils::xtime::Xtime;

#[test]
fn test_token() {
    // Create the tokens table if it doesn't exist
    db::create_token_table().expect("Failed to create tokens table");
    db::create_users_table().expect("Failed to create users table");

    // Insert a test token
    let mut user_struct = User::new(
        "test_user".to_string(),
        "test".to_string(),
        "a@a.a".to_string(),
    );
    let expering_time = Xtime::from_string("10-10-10-10-10-2027");
    let user_id = db::insert_user(&user_struct).expect("Failed to insert test user into DB");

    user_struct.uuid = user_id as u32; // Update the user_struct with the new UUID

    let token1 = token::new_user_token(user_struct.uuid, expering_time);

    // Verify the token was inserted correctly
    let result = db::get_user_by_token(&token1).expect("Failed to get user by token");

    let user = result.expect("Expected a user, but got None");
    assert_eq!(user.username, "test_user");
    assert_eq!(user.uuid, user_id as u32);
}
