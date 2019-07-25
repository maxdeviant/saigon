#![deny(missing_docs)]

//! Contains the core types for Saigon.

#[allow(missing_docs)]
pub mod content;

mod plugin;

pub use plugin::*;

/// A Saigon adapter.
pub trait Adapter {
    /// Returns the adapter's name.
    fn name(&self) -> String;

    /// Returns the adapter's version.
    fn version(&self) -> String;

    /// Handles a payload for the adapter to produce a [`Command`].
    fn handle(&mut self, payload: &str) -> Option<Command>;
}

/// The ID of a [`User`].
#[derive(Debug)]
pub struct UserId(pub String);

/// The user who sent the [`Command`].
#[derive(Debug)]
pub struct User {
    /// The user's ID.
    pub id: UserId,

    /// The user's full name.
    pub full_name: String,
}

/// A command given to a [`Plugin`].
#[derive(Debug)]
pub struct Command {
    /// The user who sent the command.
    pub user: User,

    /// The command payload.
    pub value: String,
}
