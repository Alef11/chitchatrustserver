use crate::db::db;
use crate::utils::encryption;
use crate::logger::logger::LogExpect;

use super::xtime::Xtime;

pub fn new_user_token(user_id: u32, experies_at: Xtime) -> String {
    let token = encryption::generate_token();

    db::insert_token(user_id, &token, experies_at).log_expect("Failed to insert token into database", file!());

    token
}
