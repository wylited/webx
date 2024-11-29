// src/home.rs
use vercel_runtime::{run, Body, Error, Request, Response, StatusCode};
use http::Method;

// This will embed the contents of the file as a static string at compile time
const HOME_HTML: &str = include_str!("content/home.html");

#[tokio::main]
async fn main() -> Result<(), Error> {
    run(handler).await
}

pub async fn handler(req: Request) -> Result<Response<Body>, Error> {
    match req.method() {
        &Method::GET => {
            Ok(Response::builder()
                .status(StatusCode::OK)
                .header("Content-Type", "text/html")
                .body(Body::Text(HOME_HTML.to_string()))?)
        }
        _ => Ok(Response::builder()
            .status(StatusCode::METHOD_NOT_ALLOWED)
            .body(Body::Empty)?)
    }
}
