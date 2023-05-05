use serde::Serialize;

use crate::cmd;

use super::{Command, Result};

cmd! {SET, Option<String>; key, value}

impl SetCommand {
    pub fn constraint(&mut self, cons: Constraint) -> Result<&mut Self> {
        self.set_options(cons)
    }

    pub fn get(&mut self) -> Result<&mut Self> {
        self.set_options("GET")
    }

    pub fn expire(&mut self, expire: Expire) -> Result<&mut Self> {
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
