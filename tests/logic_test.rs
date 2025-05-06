use chitchatrustserver::db::db;
use chitchatrustserver::modules::user::User;
use chitchatrustserver::utils::logics;

#[test]
fn test_login_logic() {
    db::init_db().expect("Failed to initialize database");

    let username = "test";
    let password = "password123";
    let email = "a@a.a";

    let testuser = User::new(
        username.to_string(),
        password.to_string(),
        email.to_string(),
    );

    let _ = db::insert_user(&testuser);

    let result = logics::login_logic(username, password);
    assert_eq!(result.0, true, "Login logic failed for valid credentials");
}
