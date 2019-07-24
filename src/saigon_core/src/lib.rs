#![deny(missing_docs)]

//! Contains the core types for Saigon.

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
}

/// A Saigon plugin.
pub trait Plugin {
    /// Returns the plugin's name.
    fn name(&self) -> String;

    /// Returns the plugin's version.
    fn version(&self) -> String;

    /// Receives a [`Command`] for the plugin to process.
    fn receive(&mut self, command: &Command) -> String;
}

/// A command given to a [`Plugin`].
#[derive(Debug)]
pub struct Command {
    /// The command payload.
    pub value: String,
}
