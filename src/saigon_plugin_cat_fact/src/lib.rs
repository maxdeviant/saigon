use saigon_core::content::Content;
use saigon_core::{Command, HelpText, Plugin, PluginResponse, PluginResult};
use serde::Deserialize;

pub struct CatFact;

#[derive(Deserialize)]
struct CatFactJson {
    pub fact: String,
    pub length: i32,
}

impl Plugin for CatFact {
    fn name(&self) -> String {
        env!("CARGO_PKG_NAME").into()
    }

    fn version(&self) -> String {
        env!("CARGO_PKG_VERSION").into()
    }

    fn help(&self) -> Option<HelpText> {
        Some(HelpText {
            command: "cat fact".into(),
            text: "Returns a fun fact about cats".into(),
        })
    }

    fn receive(&mut self, command: &Command) -> PluginResult {
        match command.value.as_ref() {
            "cat fact" => {
                let res: CatFactJson = reqwest::get("https://catfact.ninja/fact")
                    .unwrap()
                    .json()
                    .unwrap();

                Ok(PluginResponse::Success(Content::Text(res.fact)))
            }
            _ => Ok(PluginResponse::Ignore),
        }
    }
}
