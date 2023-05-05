use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(untagged)]
pub enum ReResponse {
    Success { result: ReSuccess },
    Error { error: String },
}

#[derive(Deserialize, Debug)]
#[serde(untagged)]
pub enum ReSuccess {
    Del(usize),
    Get(Option<String>),
    Set(Option<String>),
}
