use super::{Command, ReCmd, Result};
use reqwest::{Client, Url};
use serde::Serialize;

pub struct GetCommand {
    pub(crate) client: Client,
    pub(crate) url: Url,
    pub(crate) json: ReCmd,
}

impl Command for GetCommand {
    fn as_cmd<'a>(&'a self) -> &'a ReCmd {
        &self.json
    }

    fn set_options<'a, S: Serialize>(&'a mut self, opt: S) -> Result<&'a mut Self> {
        self.json.add_arg(opt)?;
        Ok(self)
    }

    fn client<'a>(&'a self) -> &'a Client {
        &self.client
    }

    fn url<'a>(&'a self) -> &'a Url {
        &self.url
    }
}

impl GetCommand {
    pub fn new<S: Serialize>(client: Client, url: Url, key: S) -> Result<Self> {
        let mut json = ReCmd::new("GET");
        json.add_arg(key)?;

        Ok(Self { client, url, json })
    }
}
