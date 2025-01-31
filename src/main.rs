use clang_log::init;
use http_router::prelude::*;
use std::fs;


fn error404() -> Response {
    Response::new().body(include_bytes!("www/404/index.html")).header("content-type", "text/html")
}

fn load_file(path: &String) -> Response {
    Response::new().body(fs::read(path).unwrap_or(include_bytes!("www/404/index.html").to_vec()).as_slice())
}

struct RedirectRoute;

impl Route for RedirectRoute {
    const RTYPE: RequestType = RequestType::Get;
    const PATH: &str = "/";

    fn handler(&mut self) -> fn(Request) -> Response {
        |req| {
            // Uncomment the following line and remove the already present version for a more build-optimized version.

            //let path = format!("{}{}", fs::canonicalize("./www/").expect("Wrongly configured server, directory www not found").display(), req.path);
            let path = format!("{}{}", fs::canonicalize("./src/www/").unwrap_or(fs::canonicalize("./www/").unwrap_or(fs::canonicalize(".").expect("Wrongly configured server, directory www not found"))).display(), req.path);
            let exists = fs::exists(&path).unwrap_or(false);
            if exists {
                let metadata = fs::metadata(&path).expect("MetaData could not be reached");
                if metadata.is_dir() {
                    return load_file(&format!("{}{}", path, if path.ends_with("/") {"index.html"} else {"/index.html"})).header("content-type", "text/html");
                } else if metadata.is_file() {
                    return load_file(&path);
                } else {
                    return error404();
                }
            } else {
                error404()
            }
        } 
    }
}

// If something is requested often, hard-code it for a faster response and a lesser work-load.
struct ImageRoute;

impl Route for ImageRoute {
    const RTYPE: RequestType = RequestType::Get;
    const PATH: &str = "/silly.jpg";
    fn handler(&mut self) -> fn(Request) -> Response {
        |_req| Response::new().body(include_bytes!("www/silly.jpg")).header("content-type", "image/jpeg")
    }
}

struct HomeRoute;

impl Route for HomeRoute {
    const RTYPE: RequestType = RequestType::Get;
    const PATH: &str = "/";
    fn handler(&mut self) -> fn(Request) -> Response {
        |_req| Response::new().body(include_bytes!("www/index.html")).header("content-type", "text/html")
    }
}

fn main() {
    init(log::Level::Trace, "webhost");
    let mut server = Server::new(8080, 20);
    // It checks if the requested path is hard-coded before dynamically getting the data. 
    server.add_route(ImageRoute);
    server.add_route(HomeRoute);
    server.add_default_handler(RedirectRoute);
    server.run()
}