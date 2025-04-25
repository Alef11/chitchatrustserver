use crate::utils::{
    encryption::{check_password, encrypt},
    xtime::Xtime,
};

pub struct user {
    uuid: i32,
    username: String,
    password: String,
    email: String,
    created_at: String,
    last_online: String,
    is_admin: bool,
}

impl user {
    pub fn new(username: String, password: String, email: String) -> Self {
        let encrypted_password = encrypt(&password);
        let generated_uuid = 0; //TODO: generate a unique UUID for the user
        user {
            uuid: generated_uuid,
            username,
            password: encrypted_password,
            email,
            created_at: Xtime::now().to_string(),
            last_online: Xtime::now().to_string(),
            is_admin: false,
        }
    }

    pub fn verify_password(&self, password: &str) -> bool {
        check_password(password, &self.password)
    }
}
