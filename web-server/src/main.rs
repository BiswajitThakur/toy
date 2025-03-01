use std::io;

use web_server::App;

fn main() -> io::Result<()> {
    let mut app = App::new();

    app.get("/", |req, _res| {
        println!("{:#?}", req.header);
        println!("Req Got: /");
        Ok(())
    });

    app.get("/home", |_req, _res| {
        println!("Req got: /home");
        Ok(())
    });

    app.get("/about", |_req, _res| todo!());

    app.run("0.0.0.0:8080").unwrap();

    Ok(())
}
