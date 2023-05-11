use std::fmt::Debug;

use serde::Deserialize;

// IF Result<T, String> ?

#[derive(Deserialize, Debug)]
#[serde(untagged)]
pub enum ReResponse<T> {
    Success { result: T },
    Error { error: String },
}
