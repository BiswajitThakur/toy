# Web-Server

This is a simple multi threaded easy to use [express.js](https://expressjs.com/) like web framework for [Rust](https://www.rust-lang.org/).

> :warning: Its created for learning perpose.

## Hello World Example

[more..](./examples)

```rust
use std::io;

use web_server::App;

fn main() -> io::Result<()> {
    let app = App::new();

    app.get("/", |_, res| res.send("Hello World\n"));

    app.listen("127.0.0.1:8080", |addr| {
        println!("Server Running in Addr: {}", addr);
    })?;

    Ok(())
}
```

## Todo

- [x] http
- [ ] https
