use vercel_runtime::{run, Body, Error, Request, Response, StatusCode};

use webx_api::{base, footer, header, home};

#[tokio::main]
async fn main() -> Result<(), Error> {
    run(handler).await
}

pub async fn handler(_req: Request) -> Result<Response<Body>, Error> {
    let paths = vec!["home".to_string()];
    let song = "When I 226";

    let page = base(&format!("{}{}{}", header(song), home(paths), footer()));

    Ok(Response::builder()
       .status(StatusCode::OK)
       .header("Content-Type", "text/html")
       .body(
           Body::Text(
               page
           )
       )?
    )
}
