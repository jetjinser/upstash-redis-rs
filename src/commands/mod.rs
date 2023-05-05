use async_trait::async_trait;
use reqwest::{Client, Url};
use serde::{de::DeserializeOwned, Serialize};
use serde_json::{json, Value};

use super::Result;

pub mod del;
pub mod get;
pub mod set;

#[async_trait]
pub trait Command {
    fn as_cmd(&self) -> &ReCmd;
    fn client(&self) -> &Client;
    fn url(&self) -> &Url;

    fn set_options<S: Serialize>(&mut self, opt: S) -> Result<&mut Self>;
    fn set_options_with_arg<S: Serialize>(&mut self, key: S, value: S) -> Result<&mut Self> {
        self.set_options(key)?.set_options(value)
    }

    async fn send<T: DeserializeOwned>(&self) -> Result<T> {
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

    pub fn add_arg<A: Serialize>(&mut self, arg: A) -> Result<&mut Self> {
        let v = serde_json::to_value(arg)?;
        self.elems.push(v);

        Ok(self)
    }

    pub fn add_pair<A: Serialize>(&mut self, k: A, v: A) -> Result<&mut Self> {
        let key = serde_json::to_value(k)?;
        let value = serde_json::to_value(v)?;

        self.elems.push(key);
        self.elems.push(value);

        Ok(self)
    }
}

#[macro_export]
macro_rules! cmd {
    ($s:tt,$c:ident,$($i:ident),*) => {
         pub struct $s {
            pub(crate) client: reqwest::Client,
            pub(crate) url: reqwest::Url,
            pub(crate) json: $crate::commands::ReCmd,
        }

        impl $crate::commands::Command for $s {
            fn as_cmd(&self) -> &$crate::commands::ReCmd {
                &self.json
            }

            fn set_options<S: serde::Serialize>(&mut self, opt: S) -> $crate::Result<&mut Self> {
                self.json.add_arg(opt)?;
                Ok(self)
            }

            fn client(&self) -> &reqwest::Client {
                &self.client
            }

            fn url(&self) -> &reqwest::Url {
                &self.url
            }
        }

        impl $crate::Redis {
            pub fn $c<S: serde::Serialize>(&self, $($i: S),*) -> $crate::Result<$s> {
                $s::new(self.client.clone(), self.url.clone(), $($i,)*)
            }
        }

        impl $s {
            pub fn new<S: serde::Serialize>(
                client: reqwest::Client,
                url: reqwest::Url,
                $(
                $i: S,
                )*
            ) -> $crate::Result<Self> {
                let mut json = $crate::commands::ReCmd::new(stringify!($c));

                $(
                json.add_arg($i)?;
                )*

                Ok(Self { client, url, json })
            }
        }
    };
}
