use crate::db::db;
use crate::log;
use crate::modules::user::User;
use crate::utils::encryption;

pub fn login_logic(username: &str, password: &str) -> (bool, Option<u32>) {
    let tid = match db::get_id_by_username(username) {
        Ok(Some(id)) => id,
        Ok(None) => {
            log!("User {} not found in database", username);
            return (false, None);
        }
        Err(e) => {
            log!(
                "Error occured during db lookup for user {}: {:?}",
                username,
                e
            );
            return (false, None);
        }
    };

    let pass = db::get_password_username(username).expect("Failed to get password for user");

    if encryption::check_password(password, &pass) {
        return (true, Some(tid));
    } else {
        log!("Password check failed for user {}", username);
        return (false, None);
    }
}

pub fn register_logic(username: &str, password: &str, email: &str) -> (bool, Option<u32>) {
    let user_exists = match db::check_if_user_exists(username) {
        Ok(exists) => exists,
        Err(e) => {
            log!(format!("Database error while checking user {}: {:?}", username, e).as_str());
            return (false, None);
        }
    };
    if user_exists {
        log!(format!("User {} already exists", username).as_str());
        return (false, None);
    }
    let user = User::new(
        username.to_string(),
        password.to_string(),
        email.to_string(),
    );
    let user_id = db::insert_user(&user);
    if user_id.is_ok() {
        return (true, Some(user_id.unwrap()));
    } else {
        log!(format!("Failed to create user {} in Regestration", username).as_str());
        return (false, None);
    }
}
