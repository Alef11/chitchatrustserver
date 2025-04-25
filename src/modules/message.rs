use crate::modules::user::user;

pub struct message {
    umid: i128,
    sender: user,
    receivers: Vec<user>,
}
