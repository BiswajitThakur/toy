use std::io;

use web_server::HandlerFn;

const HOST: &str = "0.0.0.0";
const PORT: u16 = 8080;

fn main() -> io::Result<()> {
    let mut handler = HandlerFn::new();
    handler.get("/", |_req, _res| todo!());
    handler.get("/home", |_req, _res| todo!());
    handler.get("/about", |_req, _res| todo!());
    handler.run(format!("{HOST}:{PORT}")).unwrap();
    Ok(())
}
