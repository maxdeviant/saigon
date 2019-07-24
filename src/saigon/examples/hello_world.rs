use saigon::BotBuilder;
use saigon_core::{Command, Plugin, Source};

pub struct HelloWorld;

impl Source for HelloWorld {
    fn name(&self) -> String {
        "HelloWorld".into()
    }

    fn version(&self) -> String {
        "1.0.0".into()
    }

    fn handle(&mut self, payload: &str) -> Option<Command> {
        Some(Command {
            value: payload.to_string(),
        })
    }
}

impl Plugin for HelloWorld {
    fn name(&self) -> String {
        "HelloWorld".into()
    }

    fn version(&self) -> String {
        "1.0.0".into()
    }

    fn receive(&mut self, command: &Command) -> String {
        format!("Hello, world! The command was: {}", command.value)
    }
}

fn main() {
    let bot = BotBuilder::new(([127, 0, 0, 1], 3000))
        .add_source(Box::new(HelloWorld))
        .add_plugin(Box::new(HelloWorld))
        .build()
        .unwrap();
    bot.start();
}
