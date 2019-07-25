use crate::content::Content;
use crate::Command;

/// A Saigon plugin.
pub trait Plugin {
    /// Returns the plugin's name.
    fn name(&self) -> String;

    /// Returns the plugin's version.
    fn version(&self) -> String;

    /// Returns the help text for the plugin.
    fn help(&self) -> Option<HelpText> {
        None
    }

    /// Receives a [`Command`] for the plugin to process.
    fn receive(&mut self, command: &Command) -> PluginResult;
}

/// The response from a [`Plugin`].
#[derive(Debug)]
pub enum PluginResponse {
    /// The plugin ignored the [`Command`].
    Ignore,

    /// The plugin successfully handled the [`Command`].
    Success(Content),
}

/// The result of a [`Plugin`] handling a [`Command`].
pub type PluginResult = Result<PluginResponse, &'static str>;

/// The help text for a [`Plugin`].
#[derive(Debug)]
pub struct HelpText {
    /// The command.
    pub command: String,

    /// The help text.
    pub text: String,
}
