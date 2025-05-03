use crate::utils::xtime::Xtime;

pub struct message {
    pub umid: i128,
    pub sender_id: i32,
    pub recipient_id: Option<i32>,
    pub group_id: Option<i32>,
    pub content: String,
    pub sent_at: Xtime,
}
