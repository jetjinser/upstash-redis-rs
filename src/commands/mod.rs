use async_trait::async_trait;
use reqwest::{Client, Url};
use serde::{de::DeserializeOwned, Serialize};
use serde_json::{json, Value};

use super::Result;

pub mod get;
pub mod set;

#[async_trait]
pub trait Command {
    fn as_cmd<'a>(&'a self) -> &'a ReCmd;
    fn client<'a>(&'a self) -> &'a Client;
    fn url<'a>(&'a self) -> &'a Url;

    fn set_options<'a, S: Serialize>(&'a mut self, opt: S) -> Result<&'a mut Self>;
    fn set_options_with_arg<'a, S: Serialize>(
        &'a mut self,
        key: S,
        value: S,
    ) -> Result<&'a mut Self> {
        self.set_options(key)?.set_options(value)
    }

    async fn send<T: DeserializeOwned>(&self) -> Result<T> {
        let json = json!(self.as_cmd());
        println!("{}", json);

        Ok(self
            .client()
            .post(self.url().as_ref())
            .json(self.as_cmd())
            .send()
            .await?
            .json()
            .await?)
    }
}

#[derive(Serialize, Debug)]
#[serde(transparent)]
pub struct ReCmd {
    elems: Vec<Value>,
}

impl ReCmd {
    pub fn new<S: Serialize>(command: S) -> Self {
        let elems = vec![json!(command)];

        Self { elems }
    }

    pub fn add_arg<'a, A: Serialize>(&'a mut self, arg: A) -> Result<&'a mut Self> {
        let v = serde_json::to_value(arg)?;
        self.elems.push(v);

        Ok(self)
    }

    pub fn add_pair<'a, A: Serialize>(&'a mut self, k: A, v: A) -> Result<&'a mut Self> {
        let key = serde_json::to_value(k)?;
        let value = serde_json::to_value(v)?;

        self.elems.push(key);
        self.elems.push(value);

        Ok(self)
    }
}
