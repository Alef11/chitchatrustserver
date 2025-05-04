use crate::db::db;
use crate::modules::user::user;
use crate::utils::encryption;

use super::xtime::Xtime;

pub fn new_user_token(user_struct: &user, experies_at: Xtime) -> String {
    let token = encryption::generate_token();

    db::insert_token(user_struct.uuid, &token, experies_at)
        .expect("Failed to insert token into database");

    token
}
