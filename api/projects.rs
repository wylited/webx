use hypertext::{html_elements, maud, Renderable, GlobalAttributes};
use vercel_runtime::{run, Body, Error, Request, Response, StatusCode};

use webx_api::{header, footer};

#[tokio::main]
async fn main() -> Result<(), Error> {
    run(handler).await
}

pub async fn handler(_req: Request) -> Result<Response<Body>, Error> {
    let paths = vec!["home".to_string(), "projects".to_string()];
    let song = "When I 226";

    let content = maud! {
        div class="border-y border-black dark:border-white-dark text-center py-4" {
            p class="text-3xl" { "Projects I've developed" }
            p { "#coming soon after proses, in the meantime just look at github pinned repositories!" }
        }
    }
    .render()
    .into_inner();

    let page = format!("{}{}{}", header(&paths, song), content, footer());

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
