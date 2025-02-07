# RoseWare RoseHost 
## (WebHost)
A both static and dynamic Website hosting server made in rust.

Powered by RoseWare Ekerö.

## Notice:
Ekerö currently doesn't support: 
- Search Queries
- HTTPS

*These features are in development.*

## Code:
```rust
use ekero::prelude::*;
use std::{fs, path::PathBuf};

struct State {
    host_path: PathBuf,
}

fn new_response() -> Response {
    Response::new().header("Server", "RoseHost/0.1.2")
}

fn new_response_body(body: &[u8]) -> Response {
    new_response()
        .body(body)
        .header("Content-Length", body.len())
}

fn error404() -> Response {
    new_response_body(include_bytes!("www/error/404.html"))
        .header("content-type", "text/html")
        .status_code(404)
}

fn error400() -> Response {
    new_response_body(include_bytes!("www/error/400.html"))
        .header("content-type", "text/html")
        .status_code(400)
}

fn error418() -> Response {
    new_response_body(include_bytes!("www/error/418.html"))
        .header("content-type", "text/html")
        .status_code(418)
}

fn error500() -> Response {
    new_response_body(include_bytes!("www/error/500.html"))
        .header("content-type", "text/html")
        .status_code(500)
}

fn load_file(path: &String) -> Response {
    match fs::read(path) {
        Err(_) => error404(),
        Ok(t) => new_response_body(t.as_slice()).status_code(202)
    }
}

// Change this "./src/www/" to "./www/" before buidling.
fn get_host_path() -> PathBuf {
    fs::canonicalize("./src/www/").expect("Wrongly configured server, folder 'www' not present.")
}

fn main() {
    let new_host_path = get_host_path();
    println!("Host Path: {}", new_host_path.display());

    clang_log::init(log::Level::Trace, "rosehost");
    let mut app = App::new("0.0.0.0:8000", 20, State { host_path: new_host_path });

    app.set_default_handler(|mut ctx| {
        let response;

        if let Ok(req) = &ctx.request() {
            // Block relative paths for server security, the program is recommended to only have read-access for the "www" folder too, remove any unecessary permissions.
            if req.path.contains("./") || req.path.contains("/.") {
                response = error418();
                response.write_to(&mut ctx)?;
                return Ok(());
            }

            let host_path = match ctx.lock_state() {
                Ok(host_path_guard) => host_path_guard.host_path.clone(),
                _ => get_host_path(),
            };

            let path = format!("{}{}", host_path.display(), req.path);
            if fs::exists(&path).unwrap_or(false) {
                if let Ok(metadata) = fs::metadata(&path) {
                    if metadata.is_dir() {
                        response = load_file(&format!(
                            "{}{}",
                            path,
                            if path.ends_with("/") {
                                "index.html"
                            } else {
                                "/index.html"
                            }
                        ))
                        .header("content-type", "text/html");
                    } else if metadata.is_file() {
                        response = load_file(&path);
                    } else {
                        response = error404();
                    }
                } else {
                    response = error500();
                }
            } else {
                response = error404();
            }
        } else {
            response = error400();
        }
        response.write_to(&mut ctx)?;
        Ok(())
    });

    // It's recommended to hard-code frequently acessed paths, they are stored during compile-time if you use "include_bytes!()".

    // It checks if the requested path is hard-coded before dynamically getting the data.
    app.get("/favicon.ico", |mut ctx| {
        let response = new_response_body(include_bytes!("www/favicon.ico"))
            .header("content-type", "image/x-icon")
            .status_code(200);
        response.write_to(&mut ctx)?;
        Ok(())
    });

    app.get("/media/silly.webp", |mut ctx| {
        let response = new_response_body(include_bytes!("www/media/silly.webp"))
            .header("content-type", "image/webp")
            .status_code(200);
        response.write_to(&mut ctx)?;
        Ok(())
    });

    app.get("/media/confused.webp", |mut ctx| {
        let response = new_response_body(include_bytes!("www/media/confused.webp"))
            .header("content-type", "image/webp")
            .status_code(200);
        response.write_to(&mut ctx)?;
        Ok(())
    });

    // This one is hard-coded due to its file-size.
    app.get("/media/catshot-roulette.webm", |mut ctx| {
        let response = new_response_body(include_bytes!("www/media/catshot-roulette.webm"))
            .header("content-type", "video/webm")
            .status_code(200);
        response.write_to(&mut ctx)?;
        Ok(())
    });

    app.get("/", |mut ctx| {
        let response = new_response_body(include_bytes!("www/index.html"))
            .header("content-type", "text/html")
            .status_code(200);
        response.write_to(&mut ctx)?;
        Ok(())
    });

    app.poll_forever()
}
```
