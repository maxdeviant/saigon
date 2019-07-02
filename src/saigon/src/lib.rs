#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

use std::net::SocketAddr;

use rocket::State;

pub trait Plugin {
    fn version(&self) -> String;
}

#[derive(Debug)]
pub struct Command {
    pub value: String,
}

#[derive(Debug)]
pub struct Message {
    pub value: String,
}

pub trait Receive {
    fn receive(&mut self, command: Command);
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
            .mount("/", routes![index])
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

    for plugin in bot.plugins.iter() {
        println!("Plugin version: {}", plugin.version());
    }

    "Hello!".into()
}
