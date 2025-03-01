use std::io;

use web_server::App;

fn main() -> io::Result<()> {
    let mut app = App::new();

    app.use_middleware(|mut req, res| {
        req.insert("hello".to_owned(), "World".to_owned());
        Ok(Some((req, res)))
    });

    app.get("/", |req, _res| {
        println!("{:#?}", req.get_header());
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
