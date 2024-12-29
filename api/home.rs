use hypertext::{html_elements, maud, Renderable, GlobalAttributes};
use vercel_runtime::{run, Body, Error, Request, Response, StatusCode};

#[tokio::main]
async fn main() -> Result<(), Error> {
    run(handler).await
}

pub async fn handler(_req: Request) -> Result<Response<Body>, Error> {
    let paths = vec!["home".to_string()];

    let content = maud! {
        fieldset #content class="border-y border-black dark:border-white-dark text-center" {
            legend class="mx-3 px-2" {
                h2 class="text-left text-purple font-mono dark:text-purple-dark" {
                    "<$> "
                        @for segment in &paths {
                            a
                                class="nav"
                                href={ "/" (segment) "#content" }
                                target="htmz"
                            {
                                (segment) "/"
                            }
                        }
                }
            }
            p class="text-3xl" { "wylited "  span class="text-xl text-gray dark:text-gray" {"- /ˈwaɪˌlaɪtɪd/"} }
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
                a class="link"
                    href="/prose#content"
                    target="htmz"
                { "prose" }
                " ~~>"
            }
            p class="font-mono" {
                "<~~ "
                a class="link" href="https://wyroam.vercel.app/" { "notes" }
                " ~~>"
            }
            p class="font-mono" {
                "<-< "
                a class="link" href="https://github.com/wylited" target="_blank" { "github" }
                " >->"
            }
            p class="font-mono" {
                "<=< "
                a class="link"
                    href="/projects#content"
                    target="htmz"
                { "projects" }
                " >=>"
            }
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
