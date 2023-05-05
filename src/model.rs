use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(untagged)]
pub enum ReResponse<T> {
    Success { result: T },
    Error { error: String },
}
