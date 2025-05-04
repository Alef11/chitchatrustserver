use crate::utils::xtime::Xtime;

pub struct Group {
    pub id: u32,
    pub name: String,
    pub owner_id: u32,
    pub created_at: Xtime,
}

pub struct GroupMember {
    pub group_id: u32,
    pub user_id: u32,
    pub joined_at: Xtime,
    pub is_admin: bool,
}
