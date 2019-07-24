use saigon_core::{Command, Source};
use serde::Deserialize;

pub struct Basecamp;

impl Source for Basecamp {
    fn name(&self) -> String {
        env!("CARGO_PKG_NAME").into()
    }

    fn version(&self) -> String {
        env!("CARGO_PKG_VERSION").into()
    }

    fn handle(&mut self, payload: &str) -> Option<Command> {
        let payload: Option<Payload> = serde_json::from_str(payload).ok();

        payload.map(|payload| Command {
            value: payload.message,
        })
    }
}

#[derive(Debug, Deserialize)]
pub struct Payload {
    pub message: String,
    pub creator: Creator,
    pub callback_url: String,
}

#[derive(Debug, Deserialize)]
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

#[derive(Debug, Deserialize)]
pub struct Company {
    pub id: i32,
    pub name: String,
}
