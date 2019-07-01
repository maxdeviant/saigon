pub struct Payload {
    pub message: String,
    pub creator: Creator,
    pub callback_url: String,
}

pub struct Creator {
    pub id: i32,
    pub attachable_sgid: String,
    pub name: String,
    pub email_address: String,
    pub personable_type: String,
    pub title: String,
    pub bio: String,
    pub created_at: String,
    pub updated_at: String,
    pub admin: bool,
    pub owner: bool,
    pub time_zone: String,
    pub avatar_url: String,
    pub company: Company,
}

pub struct Company {
    pub id: i32,
    pub name: String,
}
