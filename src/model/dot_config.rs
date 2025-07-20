use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DotConfig {
    pub content: Option<String>,
    pub src: Option<String>,
    pub dest: String,
}
