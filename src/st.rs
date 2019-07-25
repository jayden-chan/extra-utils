use futures::{future, Future};
use hyper::header::{HeaderName, HeaderValue};
use hyper::service::service_fn;
use hyper::{Body, Request, Response, Server};
use hyper::{Method, StatusCode};
use lazy_static::lazy_static;
use maplit::btreemap;
use std::collections::BTreeMap;
use std::path::Path;
use std::sync::Arc;

lazy_static! {
    static ref MIME_BY_EXTENSION: BTreeMap<String, String> = {
        let owned_version = btreemap![
            "css" => "text/css",
            "html" => "text/html",
            "js" => "text/javascript",
            "wasm" => "application/wasm",
            "woff2" => "font/woff2"
        ];

        owned_version
            .iter()
            .map(|(key, val)| (String::from(*key), String::from(*val)))
            .collect()
    };
}

// Just a simple type alias
type BoxFut = Box<Future<Item = Response<Body>, Error = hyper::Error> + Send>;

fn poor_mans_static_server(req: Request<Body>, folder: &str) -> BoxFut {
    let mut response = Response::new(Body::empty());

    match (req.method(), req.uri().path()) {
        (&Method::GET, path) => {
            // first, we serve static files
            let fs_path = match path {
                "" | "/" => format!("{}/index.html", folder),
                _ => {
                    if !path.contains(".") {
                        format!("{}{}/index.html", folder, path)
                    } else if path.ends_with("/") {
                        format!("{}{}index.html", folder, path)
                    } else {
                        format!("{}{}", folder, path)
                    }
                }
            };

            if fs_path.contains("../") {
                *response.status_mut() = StatusCode::NOT_FOUND;

                println!("[404] {}", fs_path);
                return Box::new(future::ok(response));
            }

            // Set content type here...
            let path_creator = fs_path.clone();
            let as_path = Path::new(&path_creator);

            if as_path.is_file() {
                let text = vec![std::fs::read(fs_path).unwrap()];

                if let Some(extension) = as_path.extension() {
                    if let Some(non_html_mime) = MIME_BY_EXTENSION.get(extension.to_str().unwrap())
                    {
                        (*response.headers_mut()).insert(
                            HeaderName::from_static("content-type"),
                            HeaderValue::from_static(non_html_mime),
                        );
                    };
                } else {
                    eprintln!("Content type unset for {:?}", as_path);
                }

                *response.body_mut() =
                    Body::wrap_stream(futures::stream::iter_ok::<_, ::std::io::Error>(text));
            } else {
                *response.status_mut() = StatusCode::NOT_FOUND;
            }
        }
        _ => {
            *response.status_mut() = StatusCode::NOT_FOUND;
        }
    };

    println!(
        "[{:?} {:?}] {}",
        response.version(),
        response.status(),
        req.uri().path()
    );
    Box::new(future::ok(response))
}

fn main() {
    let first_arg = std::env::args()
        .nth(1)
        .unwrap_or_else(|| String::from("3000"));

    if first_arg == "--help" || first_arg == "-h" {
        println!("st - Serve a static folder");
        println!();
        println!("Usage:");
        println!("    st <port> <folder>");
        return;
    }

    let port = first_arg.parse::<u16>().unwrap_or(3000);
    let folder = std::env::args().nth(2).unwrap_or_else(|| String::from("."));
    let folder_arc = Arc::new(folder.clone());

    let server = Server::bind(&([127, 0, 0, 1], port).into())
        .serve(move || {
            let inner = Arc::clone(&folder_arc);
            service_fn(move |req| poor_mans_static_server(req, &inner))
        })
        .map_err(|e| eprintln!("server error: {}", e));

    println!("Serving folder \"{}\" at http://localhost:{}", folder, port);
    hyper::rt::run(server)
}
