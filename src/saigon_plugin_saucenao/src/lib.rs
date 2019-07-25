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

                let mut parts: Vec<String> = Vec::new();

                parts.push("<table>".into());

                parts.push("<thead>".into());

                parts.push("<tr>".into());

                parts.push("<th>".into());
                parts.push("Title".into());
                parts.push("</th>".into());

                parts.push("<th>".into());
                parts.push("Similarity".into());
                parts.push("</th>".into());

                parts.push("</tr>".into());

                parts.push("</thead>".into());

                parts.push("<tbody>".into());

                for search_result in res.results {
                    parts.push("<tr>".into());

                    parts.push("<td>".into());
                    parts.push(
                        if let Some(ext_url) = search_result
                            .data
                            .ext_urls
                            .and_then(|ext_urls| ext_urls.into_iter().nth(0))
                        {
                            format!(
                                r#"<a href="{}">{}</a>"#,
                                ext_url,
                                search_result.data.title.unwrap_or("[no title]".into())
                            )
                        } else {
                            search_result.data.title.unwrap_or("[no title]".into())
                        },
                    );
                    parts.push("</td>".into());

                    parts.push("<td>".into());
                    parts.push(search_result.header.similarity);
                    parts.push("</td>".into());

                    parts.push("</tr>".into());
                }

                parts.push("</tbody>".into());

                parts.push("</table>".into());

                Ok(PluginResponse::Success(
                    parts.into_iter().collect::<String>(),
                ))
            }
            _ => Ok(PluginResponse::Ignore),
        }
    }
}
