use hypertext::{html_elements, maud, Renderable, GlobalAttributes};
use url::Url;
use vercel_runtime::{run, Body, Error, Request, Response, StatusCode};
use webx_api::{fetch, Prose};

#[tokio::main]
async fn main() -> Result<(), Error> {
    run(handler).await
}

pub async fn handler(req: Request) -> Result<Response<Body>, Error> {
    let paths = vec!["home".to_string(), "prose".to_string()];
    let mut url = Url::parse(&req.uri().to_string()).unwrap();

    url.set_path("/dist/prose.json");

    let json_url = url.as_str();

    let content = match fetch(&json_url) {
        Ok(json_content) => {
            match serde_json::from_str::<Vec<Prose>>(&json_content) {
                Ok(prose_list) => maud! {
                    fieldset #content class="border-y border-black dark:border-white-dark text-center max-w-prose" {
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

                        div class="flex items-center justify-between px-4 pt-4" {
                            h1 class="text-3xl" { "Prose" }
                            form action="/search#results" target="htmz" class="flex items-center" {
                                input name="query" placeholder="query" class="text-black dark:text-white bg-white dark:bg-black-dark focus:outline-none border border-gray dark:border-gray-dark rounded-l-md p-2" {}
                                button class="text-black dark:text-white rounded-r-md p-2 transition-colors duration-300 ease-in-out dark:hover:border-red border border-gray dark:border-gray-dark hover:border-red focus:outline-none focus:border-red dark:focus:border-red-dark" { "search" }
                            }
                        }
                        div #results class="flex flex-col" {
                            @for prose in prose_list.iter() {
                                div class="w-16 my-4 border-t border-blue/25 dark:border-blue-dark/25" {}
                                div class="w-full max-w-full text-left ml-4" {
                                    a class="link text-xl mb-1 hover:underline hover:decoration-[0.5px]" href=(format!("/prose/{}#content", prose.id)) target="htmz" {
                                        (prose.title.clone())
                                        span class="ml-1 text-sm text-gray dark:text-gray-dark font-mono p-0.5 outline rounded-sm outline-1 outline-gray/25 dark:outline-gray-dark/25 align-middle" {(prose.filename.clone())}
                                    }
                                    div class="flex flex-wrap gap-1 p-1" {
                                        @for tag in &prose.tags {
                                            span class="px-1 py-0.5 bg-gray/25 dark:bg-gray-dark/25 text-sm" {
                                                "#"(tag)
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }.render().into_inner(),
                Err(e) => maud! {
                    fieldset #content class="border-y border-black dark:border-white-dark text-center" {
                        legend class="mx-3 px-2" {
                            h2 class="text-left text-purple font-mono dark:text-purple-dark"{
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
                        p class="text-3xl" { "Prose" }
                        p class="text-2xl mb-8" { "Essays, Poems, and others." }
                        p class="text-red-600" { "Error parsing prose data: " (e.to_string()) }
                    }
                }.render().into_inner()
            }
        },
        Err(e) => maud! {
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
                p class="text-3xl" { "Prose" }
                p class="text-2xl mb-8" { "Essays, Poems, and others." }
                p class="text-red-600" { "Error fetching prose: " (e.to_string()) }
            }
        }.render().into_inner()
    };

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
