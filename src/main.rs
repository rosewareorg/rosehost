pub mod file;
mod html;
mod serverresource;

use ekero::prelude::*;
use file::File;
use std::{env, fs, path::PathBuf};

use html::Html;
use serverresource::ServerResource;

struct State {
    host_path: PathBuf,
}

fn new_html_response(data: &[u8], status: u16) -> Response {
    Response::new()
        .body(ServerResource::new(Html::new(data.to_vec())))
        .status_code(status)
}

fn new_bytes_response(data: &[u8], status: u16) -> Response {
    Response::new()
        .body(ServerResource::new(File {
            data: data.to_vec(),
        }))
        .status_code(status)
}

fn load_file(path: &String) -> Response {
    match fs::read(path) {
        Err(_) => new_html_response(include_bytes!("www/error/404.html"), 404),
        Ok(t) => new_bytes_response(&t, 202),
    }
}

fn main() {
    clang_log::init(log::Level::Trace, "rosehost");

    let args: Vec<String> = env::args().collect();

    let host_path = fs::canonicalize(if &args.len() > &1 { &args[1] } else { "./www/" })
        .expect("Wrongly configured server, folder 'www' or specified folder not present.");

    log::info!("Host Path: {}", host_path.display());

    let mut app = App::new("0.0.0.0:8000", 20, State { host_path });

    app.set_default_handler(|ctx| {
        let response;

        let req = ctx.request()?;
        // Block relative paths for server security,
        // the program is recommended to only have read-access for
        // the "www" directort too, remove any unecessary permissions.
        if req.path.contains("./") || req.path.contains("/.") {
            return Ok(new_html_response(include_bytes!("www/error/418.html"), 418));
        }

        let host_path = {
            let lock = ctx.state_lock()?;
            lock.host_path.clone()
        };

        let path = format!("{}{}", host_path.display(), req.path);

        if !fs::exists(&path).unwrap_or(false) {
            return Ok(new_html_response(include_bytes!("www/error/404.html"), 404));
        }

        let metadata = match fs::metadata(&path) {
            Ok(mt) => mt,
            _ => return Ok(new_html_response(include_bytes!("www/error/500.html"), 500)),
        };

        response = if metadata.is_dir() {
            load_file(&format!(
                "{}{}index.html",
                path,
                if path.ends_with("/") { "" } else { "/" }
            ))
            .header("Content-Type", "text/html")
        } else if metadata.is_file() {
            load_file(&path)
        } else {
            new_html_response(include_bytes!("www/error/404.html"), 404)
        };

        Ok(response)
    });

    // It's recommended to hard-code frequently acessed paths, they
    // are stored during compile-time if you use "include_str!()".
    // It checks if the requested path is hard-coded
    // before dynamically getting the data.
    app.get("/favicon.ico", |_ctx| {
        Ok(new_bytes_response(include_bytes!("www/favicon.ico"), 200)
            .header("Content-Type", "image/x-icon"))
    });

    app.get("/media/silly.webp", |_ctx| {
        Ok(
            new_bytes_response(include_bytes!("www/media/silly.webp"), 200)
                .header("Content-Type", "image/webp"),
        )
    });

    app.get("/media/confused.webp", |_ctx| {
        Ok(
            new_bytes_response(include_bytes!("www/media/confused.webp"), 200)
                .header("Content-Type", "image/webp"),
        )
    });

    // This one is hard-coded due to its file-size.
    app.get("/media/catshot-roulette.webm", |_ctx| {
        Ok(
            new_bytes_response(include_bytes!("www/media/catshot-roulette.webm"), 200)
                .header("Content-Type", "video/webm"),
        )
    });

    app.get("/", |_ctx| {
        Ok(new_html_response(include_bytes!("www/index.html"), 200))
    });

    app.poll_forever()
}
