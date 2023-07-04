use crate::{cmd, Command, Result};

cmd! {SPOP, Option<String>; key}

impl SpopCommand {
    pub fn add_count(&mut self, count: usize) -> Result<&mut Self> {
        self.set_options(count)
    }
}
