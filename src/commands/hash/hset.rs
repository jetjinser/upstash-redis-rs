use serde::Serialize;

use crate::{cmd, Command, Result};

cmd! {HSET, usize; key, field, value}

impl HsetCommand {
    pub fn add_field_to_value<S1, S2>(&mut self, field: S1, value: S2) -> Result<&mut Self>
    where
        S1: Serialize,
        S2: Serialize,
    {
        self.set_options(field)?.set_options(value)
    }
}
