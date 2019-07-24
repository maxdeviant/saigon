use saigon::BotBuilder;
use saigon_core::{Command, Plugin, PluginResponse, PluginResult, Source, User, UserId};

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
            user: User {
                id: UserId("1234".into()),
                full_name: "Test".into(),
            },
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

    fn receive(&mut self, command: &Command) -> PluginResult {
        Ok(PluginResponse::Success(format!(
            "Hello, world! The command was: {}",
            command.value
        )))
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
