# RoseWare WebHost
A both static and dynamic Website hosting server made in rust.

Powered by RoseWare Ekerö.


Code:
```
use ekero::{
    prelude::*,
    context::Context
};
use std::fs;


fn error404() -> Response {
    Response::new().body(include_bytes!("www/404/index.html")).header("content-type", b"text/html").status_code(404)
}

fn load_file(path: &String) -> Response {
    Response::new().body(fs::read(path).unwrap_or(include_bytes!("www/404/index.html").to_vec()).as_slice()).status_code(202)
}

fn redirect_handler(mut ctx: Context) -> Result<(), Box<(dyn std::error::Error + 'static)>> {
    let response;

    let req = &ctx.request().unwrap();
    // Uncomment the following line and remove the already present version for a more build-optimized version.

    //let path = format!("{}{}", fs::canonicalize("./www/").expect("Wrongly configured server, directory www not found").display(), req.path);
    let path = format!("{}{}", fs::canonicalize("./src/www/").unwrap_or(fs::canonicalize("./www/").unwrap_or(fs::canonicalize(".").expect("Wrongly configured server, directory www not found"))).display(), req.path);
    let exists = fs::exists(&path).unwrap_or(false);
    if exists {
        let metadata = fs::metadata(&path).expect("MetaData could not be reached");
        if metadata.is_dir() {
            response = load_file(&format!("{}{}", path, if path.ends_with("/") {"index.html"} else {"/index.html"})).header("content-type", b"text/html");
        } else if metadata.is_file() {
             response = load_file(&path);
        } else {
            response = error404();
        }
    } else {
        response = error404();
    }
    response.write_to(&mut ctx)?;
    Ok(())
}

fn main() {
    let host_path = fs::canonicalize("./src/www/").expect("Wrongly configured server, folder 'www' not present.");
    println!("Host Path: {}", host_path.display());

    clang_log::init(log::Level::Trace, "webhost");
    let mut app = App::new("0.0.0.0:8000", 20);
    // It checks if the requested path is hard-coded before dynamically getting the data. 
    app.get("/silly.jpg", |mut ctx| {
        let response = Response::new().body(include_bytes!("www/silly.jpg")).header("content-type", b"image/jpeg");
        response.write_to(&mut ctx)?;
        Ok(())
    });

    app.get("/", |mut ctx| {
        let response = Response::new().body(include_bytes!("www/index.html")).header("content-type", b"text/html");
        response.write_to(&mut ctx)?;
        Ok(())
    });

    app.set_default_handler(redirect_handler);

    app.poll_forever()
}
```
