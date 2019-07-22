#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

use std::net::SocketAddr;

use rocket::State;

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

pub struct Config {
    addr: SocketAddr,
}

pub struct Bot {
    config: Config,
    sources: Vec<Box<dyn Source + Send + Sync>>,
    plugins: Vec<Box<dyn Plugin + Send + Sync>>,
}

impl Bot {
    pub fn start(self) {
        rocket::ignite()
            .manage(self)
            .mount("/", routes![index, sources, plugins])
            .launch();
    }
}

pub struct BotBuilder {
    addr: SocketAddr,
    sources: Vec<Box<dyn Source + Send + Sync>>,
    plugins: Vec<Box<dyn Plugin + Send + Sync>>,
}

impl BotBuilder {
    pub fn new<A: Into<SocketAddr>>(addr: A) -> Self {
        Self {
            addr: addr.into(),
            sources: Vec::new(),
            plugins: Vec::new(),
        }
    }

    pub fn add_source(mut self, source: Box<dyn Source + Send + Sync>) -> Self {
        self.sources.push(source);
        self
    }

    pub fn add_plugin(mut self, plugin: Box<dyn Plugin + Send + Sync>) -> Self {
        self.plugins.push(plugin);
        self
    }

    pub fn build(self) -> Result<Bot, &'static str> {
        Ok(Bot {
            config: Config { addr: self.addr },
            sources: self.sources,
            plugins: self.plugins,
        })
    }
}

#[post("/", data = "<payload>")]
fn index(bot: State<Bot>, payload: String) -> String {
    println!("Payload is {}", payload);

    let command = bot
        .sources
        .iter()
        .find_map(|source| source.handle(&payload));

    println!("Command is {:?}", &command);

    if let Some(command) = command {
        for plugin in bot.plugins.iter() {
            let response = plugin.receive(&command);
            println!("Response from {} was {}", plugin.name(), response);
        }
    }

    let command = Command { value: payload };

    for plugin in bot.plugins.iter() {
        let response = plugin.receive(&command);
        println!("Response from {} was {}", plugin.name(), response);
    }

    "Hello!".into()
}

#[get("/sources")]
fn sources(bot: State<Bot>) -> String {
    bot.sources
        .iter()
        .map(|source| format!("{}: v{}\n", source.name(), source.version()))
        .collect::<String>()
}

#[get("/plugins")]
fn plugins(bot: State<Bot>) -> String {
    bot.plugins
        .iter()
        .map(|plugin| format!("{}: v{}\n", plugin.name(), plugin.version()))
        .collect::<String>()
}
