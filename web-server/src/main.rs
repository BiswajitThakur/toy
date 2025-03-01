use std::io;

use web_server::{App, Status};

fn main() -> io::Result<()> {
    let mut app = App::new();

    app.use_middleware(|mut req, res| {
        req.insert_header("hello".to_owned(), "World".to_owned());
        Ok(Some((req, res)))
    });

    app.get("/", |req, res| {
        println!("{:#?}", req.get_headers());
        println!("Req Got: /");

        res.status(Status::OK);
        res.send("Hello World")?;
        Ok(())
    });

    app.post("/", |req, _res| {
        println!("{:#?}", req.get_headers());
        println!("Req Post: /");
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
