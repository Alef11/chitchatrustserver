use crate::utils::xtime::Xtime;

pub struct Messages {
    pub umid: u128,
    pub sender_id: u32,
    pub recipient_id: Option<u32>,
    pub group_id: Option<u32>,
    pub content: String,
    pub sent_at: Xtime,
}
