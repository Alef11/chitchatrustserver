use crate::db::db;
use crate::utils::encryption;
use crate::modules::user::User;
use crate::log;

pub fn login_logic(username: &str, password: &str) -> (bool, Option<u32>) {
    let tid = match db::get_id_by_username(username) {
        Ok(Some(id)) => id,        // Extract the user ID if it exists
        _ => return (false, None), // Return false if no ID is found or an error occurs
    };

    let user = match db::get_user_by_id(tid) {
        Ok(Some(user)) => user,    // Extract the User if it exists
        _ => return (false, None), // Return false if no User is found or an error occurs
    };
    if encryption::check_password(password, &user.password) {
        return (true, Some(tid));
    } else {
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
    let hashed_password = encryption::encrypt(password);
    let user = User::new(username.to_string(), hashed_password, email.to_string());
    let user_id = db::insert_user(&user);
    if user_id.is_ok() {
        return (true, Some(user_id.unwrap()));
    } else {
        log!(format!("Failed to create user {} in Regestration", username).as_str());
        return (false, None);
    }
}
