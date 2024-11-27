use vercel_runtime::{run, Body, Error, Request, Response, StatusCode};
use serde_json::json;
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
            let response = json!({ "count": count });
            Ok(Response::builder()
                .status(StatusCode::OK)
                .header("Content-Type", "application/json")
                .body(Body::Text(response.to_string()))?)
        }
        &Method::POST => {
            let new_count = COUNTER.fetch_add(1, Ordering::Relaxed) + 1;
            let response = json!({ "count": new_count });
            Ok(Response::builder()
                .status(StatusCode::OK)
                .header("Content-Type", "application/json")
                .body(Body::Text(response.to_string()))?)
        }
        _ => Ok(Response::builder()
            .status(StatusCode::METHOD_NOT_ALLOWED)
            .body(Body::Empty)?)
    }
}
