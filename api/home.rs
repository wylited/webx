use hypertext::{html_elements, maud, Renderable, GlobalAttributes};
use vercel_runtime::{run, Body, Error, Request, Response, StatusCode};

#[tokio::main]
async fn main() -> Result<(), Error> {
    run(handler).await
}

pub async fn handler(_req: Request) -> Result<Response<Body>, Error> {
    let paths = vec!["home".to_string()];

    let content = maud! {
        fieldset #content class="border-y border-black dark:border-white-dark text-center pb-2" {
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
            p class="text-3xl" {
                "wylited "
                span class="group relative" {
                    span class="text-base text-gray dark:text-gray-dark inline-block transition-opacity duration-300 group-hover:opacity-0" {"- /ˈwaɪˌlaɪtɪd/"}
                    span class="text-base text-gray dark:text-gray-dark absolute left-0 bottom-0 inline-block opacity-0 transition-opacity duration-300 group-hover:opacity-100" {
                        "- વાયલાઇટેડ"
                    }
                }
            }
            p class="text-white dark:text-black-dark" { "dhairya" }
            div class="group relative text-center w-full mb-1" {
                span class="inline-block transition-opacity duration-300 group-hover:opacity-0 w-full" {
                    "International Baccalaureate 'm25"
                }
                span class="absolute left-0 top-0 inline-block opacity-0 transition-opacity duration-300 group-hover:opacity-100 w-full" {
                    "Math AA Hl, Physics Hl, Comp Sci Hl, Econ Hl"
                }
            }
            p class="mb-2 relative inline-block after:content-[''] after:absolute after:bottom-0 after:left-0 after:w-[49%] after:h-[4px] after:bg-orange dark:after:bg-orange-dark" {
                "Now Reading: Freakonomics, 49%"
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
