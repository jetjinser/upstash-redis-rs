use commands::{get::GetCommand, set::SetCommand};
use reqwest::{header, Client, Url};

pub mod commands;
mod error;

pub use commands::Command;
pub use error::Result;
use serde::Serialize;

pub struct Redis {
    client: Client,
    url: Url,
}

impl Redis {
    pub fn new<RS>(url: RS, token: RS) -> Result<Self>
    where
        RS: AsRef<str>,
    {
        let auth = format!("Bearer {}", token.as_ref());

        let mut value = header::HeaderValue::from_str(auth.as_str())?;
        value.set_sensitive(true);

        let mut headers = header::HeaderMap::new();
        headers.append(header::AUTHORIZATION, value);

        let client = Client::builder().default_headers(headers).build()?;

        let url = Url::parse(url.as_ref())?;

        Ok(Self { client, url })
    }

    pub fn get<S: Serialize>(&self, key: S) -> Result<GetCommand> {
        GetCommand::new(self.client.clone(), self.url.clone(), key)
    }

    pub fn set<S: Serialize>(&self, key: S, value: S) -> Result<SetCommand> {
        SetCommand::new(self.client.clone(), self.url.clone(), key, value)
    }
}
