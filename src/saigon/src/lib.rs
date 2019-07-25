#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

use std::net::SocketAddr;

use log::{debug, LevelFilter};
use parking_lot::RwLock;
use rocket::State;
use saigon_core::{HelpText, Plugin, PluginResponse, Source};

pub type BoxedSource = Box<dyn Source + Send + Sync>;

pub type BoxedPlugin = Box<dyn Plugin + Send + Sync>;

pub struct Config {
    log_level: LevelFilter,
    addr: SocketAddr,
}

pub struct Bot {
    config: Config,
    sources: Vec<BoxedSource>,
    plugins: Vec<BoxedPlugin>,
}

impl Bot {
    pub fn start(self) {
        self.configure_logger()
            .expect("Failed to configure logging");

        std::env::set_var("ROCKET_ADDRESS", format!("{}", self.config.addr.ip()));
        std::env::set_var("ROCKET_PORT", format!("{}", self.config.addr.port()));

        rocket::ignite()
            .manage(RwLock::new(self))
            .mount("/", routes![index, sources, plugins])
            .launch();
    }

    fn configure_logger(&self) -> Result<(), log::SetLoggerError> {
        use fern::colors::{Color, ColoredLevelConfig};

        let colors = ColoredLevelConfig::new()
            .error(Color::Magenta)
            .warn(Color::Yellow)
            .info(Color::Blue)
            .debug(Color::Cyan)
            .trace(Color::Green);

        fern::Dispatch::new()
            .format(move |out, message, record| {
                out.finish(format_args!(
                    "[{}][{}] {}",
                    record.target(),
                    colors.color(record.level()),
                    message
                ))
            })
            .level(self.config.log_level)
            .chain(std::io::stdout())
            .apply()
    }
}

pub struct BotBuilder {
    log_level: LevelFilter,
    addr: SocketAddr,
    sources: Vec<BoxedSource>,
    plugins: Vec<BoxedPlugin>,
}

impl BotBuilder {
    pub fn new<A: Into<SocketAddr>>(addr: A) -> Self {
        Self {
            addr: addr.into(),
            ..Default::default()
        }
    }

    pub fn log_level(mut self, level: LevelFilter) -> Self {
        self.log_level = level;
        self
    }

    pub fn add_source(mut self, source: BoxedSource) -> Self {
        self.sources.push(source);
        self
    }

    pub fn add_plugin(mut self, plugin: BoxedPlugin) -> Self {
        self.plugins.push(plugin);
        self
    }

    pub fn build(self) -> Result<Bot, &'static str> {
        Ok(Bot {
            config: Config {
                log_level: self.log_level,
                addr: self.addr,
            },
            sources: self.sources,
            plugins: self.plugins,
        })
    }
}

impl Default for BotBuilder {
    fn default() -> Self {
        Self {
            log_level: LevelFilter::Info,
            addr: ([127, 0, 0, 1], 3000).into(),
            sources: Vec::new(),
            plugins: Vec::new(),
        }
    }
}

#[post("/", data = "<payload>")]
fn index(bot: State<RwLock<Bot>>, payload: String) -> String {
    debug!(target: "saigon", "Payload is {}", payload);

    let command = {
        let mut bot = bot.write();

        bot.sources
            .iter_mut()
            .find_map(|source| source.handle(&payload))
    };

    debug!(target: "saigon", "Command is {:?}", &command);

    if let Some(command) = command {
        if command.value.to_lowercase() == "help" {
            let mut help_texts = bot
                .read()
                .plugins
                .iter()
                .filter_map(|plugin| plugin.help())
                .collect::<Vec<HelpText>>();

            help_texts.insert(
                0,
                HelpText {
                    command: "help".into(),
                    text: "Displays help information".into(),
                },
            );

            let mut parts: Vec<String> = Vec::new();

            parts.push("<table>".into());

            parts.push("<thead>".into());

            parts.push("<tr>".into());
            parts.push("<th>".into());
            parts.push("Command".into());
            parts.push("</th>".into());
            parts.push("<th>".into());
            parts.push("Description".into());
            parts.push("</th>".into());
            parts.push("</tr>".into());

            parts.push("</thead>".into());

            parts.push("<tbody>".into());

            for help in help_texts {
                parts.push("<tr>".into());
                parts.push("<td>".into());
                parts.push("<code>".into());
                parts.push(help.command);
                parts.push("</code>".into());
                parts.push("</td>".into());
                parts.push("<td>".into());
                parts.push(help.text);
                parts.push("</td>".into());
                parts.push("</tr>".into());
            }

            parts.push("</tbody>".into());

            parts.push("</table>".into());

            return parts.into_iter().collect::<String>();
        }

        let mut bot = bot.write();

        bot.plugins
            .iter_mut()
            .filter_map(|plugin| plugin.receive(&command).ok())
            .filter_map(|res| match res {
                PluginResponse::Success(res) => Some(res),
                PluginResponse::Ignore => None,
            })
            .collect::<String>()
    } else {
        "NO COMMAND".into()
    }
}

#[get("/sources")]
fn sources(bot: State<RwLock<Bot>>) -> String {
    bot.read()
        .sources
        .iter()
        .map(|source| format!("{}: v{}\n", source.name(), source.version()))
        .collect::<String>()
}

#[get("/plugins")]
fn plugins(bot: State<RwLock<Bot>>) -> String {
    bot.read()
        .plugins
        .iter()
        .map(|plugin| format!("{}: v{}\n", plugin.name(), plugin.version()))
        .collect::<String>()
}
