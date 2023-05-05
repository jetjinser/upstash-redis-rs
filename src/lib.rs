#![feature(macro_metavar_expr)]

use reqwest::{header, Client, Url};

pub mod commands;
mod error;
mod model;

pub use commands::Command;
pub use error::Result;

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
}
