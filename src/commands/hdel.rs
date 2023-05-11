use serde::Serialize;

use crate::{cmd, Command, Result};

cmd! {HDEL, usize; key, field}

impl HdelCommand {
    pub fn add_field<S: Serialize>(&mut self, field: S) -> Result<&mut Self> {
        self.set_options(field)
    }
}
