use std::io;

use web_server::App;

fn main() -> io::Result<()> {
    let app = App::new();

    app.get("/", |_, res| res.send("Hello World\n"));

    app.listen("127.0.0.1:8080", |addr| {
        println!("Server Running in Addr: {}", addr);
    })
    .unwrap();

    Ok(())
}
