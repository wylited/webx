use hypertext::{html_elements, maud, Renderable, GlobalAttributes};
use vercel_runtime::{run, Body, Error, Request, Response, StatusCode};

use webx_api::{header, footer, ExtraAttributes};

#[tokio::main]
async fn main() -> Result<(), Error> {
    run(handler).await
}

pub async fn handler(_req: Request) -> Result<Response<Body>, Error> {
    let paths = vec!["home".to_string()];
    let song = "When I 226";

    let content = maud! {
        div class="border-y border-black dark:border-white-dark text-center py-4" {
            p class="text-3xl" { "wylited" }
            p class="text-white dark:text-black-dark" { "dhairya" }
            p { "International Baccalaureate 'm25" }
            p class="text-white dark:text-black-dark" {
                "Math AA Hl, Physics Hl, Comp Sci Hl, Econ Hl"
            }
            h2 class="text-2xl text-yellow dark:text-yellow-dark py-2" {
                "quick links"
            }
            p class="font-mono" {
                "<~~ "
                button class="link"
                    hx-get="/api/prose"
                    hx-target="#content"
                { "prose" }
                " ~~>"
            }
            p class="font-mono" {
                "<~~ "
                button class="link" { "notes" }
                " ~~>"
            }
            p class="font-mono" {
                "<-< "
                a class="link" href="https://github.com/wylited" target="_blank" { "github" }
                " >->"
            }
            p class="font-mono" {
                "<=< "
                button class="link"
                    hx-get="/api/projects"
                    hx-target="#content"
                { "projects" }
                " >=>"
            }
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
