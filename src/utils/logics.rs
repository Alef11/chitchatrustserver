use crate::db::db;
use crate::utils::encryption;

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
