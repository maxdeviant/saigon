#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

use std::net::SocketAddr;

use rocket::State;

pub trait Plugin {
    fn name(&self) -> String;

    fn version(&self) -> String;

    fn receive(&self, command: &Command) -> String;
}

#[derive(Debug)]
pub struct Command {
    pub value: String,
}

pub struct Config {
    addr: SocketAddr,
}

pub struct Bot {
    config: Config,
    plugins: Vec<Box<dyn Plugin + Send + Sync>>,
}

impl Bot {
    pub fn start(self) {
        rocket::ignite()
            .manage(self)
            .mount("/", routes![index, plugins])
            .launch();
    }
}

pub struct BotBuilder {
    addr: SocketAddr,
    plugins: Vec<Box<dyn Plugin + Send + Sync>>,
}

impl BotBuilder {
    pub fn new<A: Into<SocketAddr>>(addr: A) -> Self {
        Self {
            addr: addr.into(),
            plugins: Vec::new(),
        }
    }

    pub fn add_plugin(mut self, plugin: Box<dyn Plugin + Send + Sync>) -> Self {
        self.plugins.push(plugin);
        self
    }

    pub fn build(self) -> Result<Bot, &'static str> {
        Ok(Bot {
            config: Config { addr: self.addr },
            plugins: self.plugins,
        })
    }
}

#[post("/", data = "<payload>")]
fn index(bot: State<Bot>, payload: String) -> String {
    println!("Payload is {}", payload);

    let command = Command { value: payload };

    for plugin in bot.plugins.iter() {
        let response = plugin.receive(&command);
        println!("Response from {} was {}", plugin.name(), response);
    }

    "Hello!".into()
}

#[get("/plugins")]
fn plugins(bot: State<Bot>) -> String {
    bot.plugins
        .iter()
        .map(|plugin| format!("{}: v{}", plugin.name(), plugin.version()))
        .collect::<String>()
}
