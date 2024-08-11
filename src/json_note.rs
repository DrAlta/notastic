use serde::Deserialize;

#[derive(Debug, PartialEq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct JsonNote {
    pub uuid: String,
    pub title: String,
    pub body: String,
    #[serde(rename = "bodyHistory")]
    pub body_history: Vec<String>,
}