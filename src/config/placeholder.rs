use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Placeholder {
    pub sources: Option<Vec<InputSource>>,
    pub callback: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum InputSource {
    Text(String),
    Script(String),
}
