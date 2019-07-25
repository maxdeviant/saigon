use saigon_core::content::{Content, Link, Table, TableColumn, TableRow};
use saigon_core::{Command, HelpText, Plugin, PluginResponse, PluginResult};
use serde::Deserialize;

#[derive(Deserialize)]
struct SearchResponse {
    pub results: Vec<SearchResult>,
}

#[derive(Deserialize)]
struct SearchResult {
    pub header: ImageHeader,
    pub data: ImageData,
}

#[derive(Deserialize)]
struct ImageHeader {
    pub similarity: String,
}

#[derive(Deserialize)]
struct ImageData {
    pub ext_urls: Option<Vec<String>>,
    pub title: Option<String>,
}

pub struct SauceNao;

impl Plugin for SauceNao {
    fn name(&self) -> String {
        env!("CARGO_PKG_NAME").into()
    }

    fn version(&self) -> String {
        env!("CARGO_PKG_VERSION").into()
    }

    fn help(&self) -> Option<HelpText> {
        Some(HelpText {
            command: "saucenao &lt;image_url&gt;".into(),
            text: "Returns the sauce of an image".into(),
        })
    }

    fn receive(&mut self, command: &Command) -> PluginResult {
        match &command.value {
            command if command.starts_with("saucenao") => {
                let image_url = command.split(" ").collect::<Vec<&str>>()[1];

                let res: SearchResponse = reqwest::get(&format!(
                    "https://saucenao.com/search.php?output_type=2&url={}",
                    image_url
                ))
                .unwrap()
                .json()
                .unwrap();

                let mut table = Table::new();
                table.header.add_row(TableRow {
                    columns: vec![
                        TableColumn {
                            value: Content::Text("Title".into()),
                        },
                        TableColumn {
                            value: Content::Text("Similarity".into()),
                        },
                    ],
                });

                for search_result in res.results {
                    let mut row = TableRow::new();

                    row.add_column(TableColumn::new(
                        if let Some(ext_url) = search_result
                            .data
                            .ext_urls
                            .and_then(|ext_urls| ext_urls.into_iter().nth(0))
                        {
                            Content::Link(Box::new(Link {
                                url: ext_url,
                                text: Content::Text(
                                    search_result.data.title.unwrap_or("[no title]".into()),
                                ),
                            }))
                        } else {
                            Content::Text(search_result.data.title.unwrap_or("[no title]".into()))
                        },
                    ));

                    row.add_column(TableColumn::new(Content::Text(
                        search_result.header.similarity,
                    )));

                    table.body.add_row(row);
                }

                Ok(PluginResponse::Success(Content::Table(Box::new(table))))
            }
            _ => Ok(PluginResponse::Ignore),
        }
    }
}
