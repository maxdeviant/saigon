#![deny(missing_docs)]

//! Contains the core types for Saigon.

/// The result of a [`Plugin`] handling a [`Command`].
pub type PluginResult = Result<PluginResponse, &'static str>;

/// A Saigon source.
pub trait Source {
    /// Returns the source's name.
    fn name(&self) -> String;

    /// Returns the source's version.
    fn version(&self) -> String;

    /// Handles a payload for the source to produce a [`Command`].
    fn handle(&mut self, payload: &str) -> Option<Command>;
}

/// The response from a [`Plugin`].
pub enum PluginResponse {
    /// The plugin ignored the [`Command`].
    Ignore,

    /// The plugin successfully handled the [`Command`].
    Success(String),
}

/// A Saigon plugin.
pub trait Plugin {
    /// Returns the plugin's name.
    fn name(&self) -> String;

    /// Returns the plugin's version.
    fn version(&self) -> String;

    /// Receives a [`Command`] for the plugin to process.
    fn receive(&mut self, command: &Command) -> PluginResult;
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
