pub trait Source {
    fn name(&self) -> String;

    fn version(&self) -> String;

    fn handle(&self, payload: &String) -> Option<Command>;
}

pub enum PluginResponse {
    Ignore,
}

pub trait Plugin {
    fn name(&self) -> String;

    fn version(&self) -> String;

    fn receive(&self, command: &Command) -> String;
}

#[derive(Debug)]
pub struct Command {
    pub value: String,
}
