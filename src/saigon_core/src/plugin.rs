use crate::Command;

/// A Saigon plugin.
pub trait Plugin {
    /// Returns the plugin's name.
    fn name(&self) -> String;

    /// Returns the plugin's version.
    fn version(&self) -> String;

    /// Returns the help text for the plugin.
    fn help(&self) -> Option<String> {
        None
    }

    /// Receives a [`Command`] for the plugin to process.
    fn receive(&mut self, command: &Command) -> PluginResult;
}

/// The response from a [`Plugin`].
pub enum PluginResponse {
    /// The plugin ignored the [`Command`].
    Ignore,

    /// The plugin successfully handled the [`Command`].
    Success(String),
}

/// The result of a [`Plugin`] handling a [`Command`].
pub type PluginResult = Result<PluginResponse, &'static str>;
