use vercel_runtime::{run, Body, Error, Request, Response, StatusCode};
use std::sync::atomic::{AtomicU64, Ordering};
use http::Method;

static COUNTER: AtomicU64 = AtomicU64::new(0);

#[tokio::main]
async fn main() -> Result<(), Error> {
    run(handler).await
}

pub async fn handler(req: Request) -> Result<Response<Body>, Error> {
    match req.method() {
        &Method::GET => {
            let count = COUNTER.load(Ordering::Relaxed);
            Ok(Response::builder()
                .status(StatusCode::OK)
                .header("Content-Type", "text/plain")
                .body(Body::Text(count.to_string()))?)
        }
        &Method::POST => {
            let new_count = COUNTER.fetch_add(1, Ordering::Relaxed) + 1;
            Ok(Response::builder()
                .status(StatusCode::OK)
                .header("Content-Type", "text/plain")
                .body(Body::Text(new_count.to_string()))?)
        }
        _ => Ok(Response::builder()
            .status(StatusCode::METHOD_NOT_ALLOWED)
            .body(Body::Empty)?)
    }
}
