use hypertext::{html_elements, maud, Renderable, GlobalAttributes};
use vercel_runtime::{run, Body, Error, Request, Response, StatusCode};


#[tokio::main]
async fn main() -> Result<(), Error> {
    run(handler).await
}

pub async fn handler(_req: Request) -> Result<Response<Body>, Error> {
    let paths = vec!["home".to_string(), "projects".to_string()];
    let song = "When I 226";

    let content = maud! {
        fieldset #content class="border-y border-black dark:border-white-dark text-center py-4" {
            legend class="mx-3 px-2" {
                h2 class="text-left text-purple font-mono dark:text-purple-dark" {
                    "<$> "
                        @for segment in paths.clone() {
                            a
                                class="nav"
                                href={ "/" (segment.clone()) "#content" }
                                target="htmz"
                            {
                                (segment) "/"
                            }
                        }
                }
            }
            p class="text-3xl" { "Projects I've developed" }
            p { "#coming soon after proses, in the meantime just look at github pinned repositories!" }
        }
    }
    .render()
    .into_inner();

    Ok(Response::builder()
       .status(StatusCode::OK)
       .header("Content-Type", "text/html")
       .body(
           Body::Text(
               content
           )
       )?
    )
}
