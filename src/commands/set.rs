use reqwest::{Client, Url};
use serde::Serialize;

use super::{Command, ReCmd, Result};

pub struct SetCommand {
    pub(crate) client: Client,
    pub(crate) url: Url,
    pub(crate) json: ReCmd,
}

impl Command for SetCommand {
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

impl SetCommand {
    pub fn new<S: Serialize>(client: Client, url: Url, key: S, value: S) -> Result<Self> {
        let mut json = ReCmd::new("SET");
        json.add_arg(key)?.add_arg(value)?;

        Ok(Self { client, url, json })
    }

    pub fn constraint<'a>(&'a mut self, cons: Constraint) -> Result<&'a mut Self> {
        self.set_options(cons)
    }

    pub fn get<'a>(&'a mut self) -> Result<&'a mut Self> {
        self.set_options("GET")
    }

    pub fn expire<'a>(&'a mut self, expire: Expire) -> Result<&'a mut Self> {
        match expire {
            Expire::EX(s) => self.set_options("EX")?.set_options(s),
            Expire::PX(s) => self.set_options("PX")?.set_options(s),
            Expire::EXAT(s) => self.set_options("EXAT")?.set_options(s),
            Expire::PXAT(s) => self.set_options("PXAT")?.set_options(s),
            Expire::KEEPTTL => self.set_options("KEEPTTL"),
        }
    }
}

#[derive(Serialize)]
pub enum Constraint {
    NX,
    XX,
}

#[derive(Serialize)]
pub enum Expire {
    EX(usize),
    PX(usize),
    EXAT(usize),
    PXAT(usize),
    KEEPTTL,
}
