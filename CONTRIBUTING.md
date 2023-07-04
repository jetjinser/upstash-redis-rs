# Contribution guidelines

There is no such thing yet.

## code structure

```
src
├── commands
│  ├── bitmap/
│  ├── connection/
│  ├── generic/
│  ├── geo/
│  ├── hash/
│  ├── hyper_log_log/
│  ├── json/
│  ├── list/
│  ├── mod.rs
│  ├── pub_sub/
│  ├── scripting/
│  ├── server/
│  ├── set/
│  ├── sorted_set/
│  ├── streams/
│  ├── string/
│  └── transactions/
├── error.rs
├── lib.rs
└── model.rs
```

All redis commands defined in `commands/<group>/` folder.
Example:

```
src/commands/hash
├── hdel.rs
├── hget.rs
├── hgetall.rs
├── hlen.rs
├── hset.rs
└── mod.rs
```

`hdel.rs` may looks like this:
```rust
use serde::Serialize;

use crate::{cmd, Command, Result};

cmd! {HDEL, usize; key, field}

impl HdelCommand {
    pub fn add_field<S: Serialize>(&mut self, field: S) -> Result<&mut Self> {
        self.set_options(field)
    }
}
```

`cmd!` macro defined A `HDEL` command that return `usize`, need params `key` and `field`.
It will generate a struct named `HdelCommand`.

`add_field` method add a option param `field`.

`HDEL` in redis syntax description:
```
HDEL key field [field ...]
```

## implement a command

implement [xdel](https://redis.io/commands/xdel/) as a example.

1. add `pub mod xdel;` to `src/commands/streams/mod.rs`.
2. create `xdel.rs` in `src/commands/streams/`, becase `xdel` belongs to `streams` group.
3. define `xdel` command in `src/commands/streams/xdel.rs`.

`xdel.rs`:
```rust
use serde::Serialize;

use crate::{cmd, Command, Result};

cmd! {XDEL, usize; key, id}

impl XdelCommand {
    pub fn add_id<S: Serialize>(&mut self, id: S) -> Result<&mut Self> {
        self.set_options(id)
    }
}
```
