use saigon::{BotBuilder, Command, Plugin};

pub struct HelloWorld {}

impl Plugin for HelloWorld {
    fn name(&self) -> String {
        "HelloWorld".into()
    }

    fn version(&self) -> String {
        "1.0.0".into()
    }

    fn receive(&self, command: &Command) -> String {
        format!("Hello, world! The command was: {}", command.value)
    }
}

fn main() {
    let bot = BotBuilder::new(([127, 0, 0, 1], 3000))
        .add_plugin(Box::new(HelloWorld {}))
        .build()
        .unwrap();
    bot.start();
}
