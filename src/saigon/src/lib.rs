#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

use std::net::SocketAddr;

use log::{debug, LevelFilter};
use parking_lot::RwLock;
use rocket::State;
use saigon_core::content::Content;
use saigon_core::{Adapter, HelpText, Plugin, PluginResponse};

pub type BoxedAdapter = Box<dyn Adapter + Send + Sync>;

pub type BoxedPlugin = Box<dyn Plugin + Send + Sync>;

pub struct Config {
    log_level: LevelFilter,
    addr: SocketAddr,
}

pub struct Bot {
    config: Config,
    adapters: Vec<BoxedAdapter>,
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
            .mount("/", routes![index, adapters, plugins])
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
    adapters: Vec<BoxedAdapter>,
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

    pub fn add_source(mut self, source: BoxedAdapter) -> Self {
        self.adapters.push(source);
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
            adapters: self.adapters,
            plugins: self.plugins,
        })
    }
}

impl Default for BotBuilder {
    fn default() -> Self {
        Self {
            log_level: LevelFilter::Info,
            addr: ([127, 0, 0, 1], 3000).into(),
            adapters: Vec::new(),
            plugins: Vec::new(),
        }
    }
}

#[post("/", data = "<payload>")]
fn index(bot: State<RwLock<Bot>>, payload: String) -> String {
    debug!(target: "saigon", "Payload is {}", payload);

    let command = {
        let mut bot = bot.write();

        bot.adapters
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

            use saigon_core::content::{Content, Table, TableColumn, TableRow};

            let mut table = Table::new();

            table.header.add_row(TableRow {
                columns: vec![
                    TableColumn::new(Content::Text("Command".into())),
                    TableColumn::new(Content::Text("Description".into())),
                ],
            });

            for help in help_texts {
                table.body.add_row(TableRow {
                    columns: vec![
                        TableColumn::new(Content::Text(format!("<code>{}</code>", help.command))),
                        TableColumn::new(Content::Text(help.text)),
                    ],
                });
            }

            return to_html_string(Content::Table(Box::new(table)));
        }

        let mut bot = bot.write();

        bot.plugins
            .iter_mut()
            .filter_map(|plugin| plugin.receive(&command).ok())
            .filter_map(display_response)
            .collect::<String>()
    } else {
        "NO COMMAND".into()
    }
}

fn to_html_string(content: Content) -> String {
    match content {
        Content::Fragment(contents) => contents.into_iter().map(to_html_string).collect::<String>(),
        Content::Text(value) => value,
        Content::Link(link) => format!(
            "<a href=\"{}\">{}</a>",
            link.url.clone(),
            to_html_string(link.text)
        ),
        Content::Table(table) => {
            let mut parts: Vec<String> = Vec::new();

            parts.push("<table>".into());

            parts.push("<thead>".into());

            for row in table.header.rows {
                parts.push("<tr>".into());

                for column in row.columns {
                    parts.push("<th>".into());
                    parts.push(to_html_string(column.value));
                    parts.push("</th>".into());
                }

                parts.push("</tr>".into());
            }

            parts.push("</thead>".into());

            parts.push("<tbody>".into());

            for row in table.body.rows {
                parts.push("<tr>".into());

                for column in row.columns {
                    parts.push("<td>".into());
                    parts.push(to_html_string(column.value));
                    parts.push("</td>".into());
                }

                parts.push("</tr>".into());
            }

            parts.push("</tbody>".into());

            parts.push("</table>".into());

            parts.into_iter().collect::<String>()
        }
    }
}

fn display_response(response: PluginResponse) -> Option<String> {
    match response {
        PluginResponse::Success(content) => Some(to_html_string(content)),
        PluginResponse::Ignore => None,
    }
}

#[get("/adapters")]
fn adapters(bot: State<RwLock<Bot>>) -> String {
    bot.read()
        .adapters
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
