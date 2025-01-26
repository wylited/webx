use http::Uri;
use hypertext::{html_elements, maud, Renderable, GlobalAttributes};
use url::Url;
use vercel_runtime::{run, Body, Error, Request, Response, StatusCode};
use webx_api::{fetch, Prose};

#[tokio::main]
async fn main() -> Result<(), Error> {
    run(handler).await
}

// Handler function for the server
pub async fn handler(req: Request) -> Result<Response<Body>, Error> {
    let base_path = vec!["home".to_string(), "prose".to_string()];

    // Generate the JSON URL from the request
    let json_url = build_json_url(req.uri())?;

    // Fetch and process the Prose content
    let content = match fetch_prose_data(&json_url).await {
        Ok(prose_list) => render_prose_view(&base_path, prose_list),
        Err(err) => render_error_view(&base_path, "Error fetching prose", &err.to_string()),
    };

    // Return the rendered HTML as a response
    Ok(Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "text/html")
        .body(Body::Text(content))?)
}

fn build_json_url(uri: &Uri) -> Result<String, Error> {
    let mut url = Url::parse(&uri.to_string())?;
    url.set_path("/dist/prose.json");
    Ok(url.to_string())
}

async fn fetch_prose_data(json_url: &str) -> Result<Vec<Prose>, Error> {
    let json_content = fetch(json_url)?;
    let prose_list: Vec<Prose> = serde_json::from_str(&json_content)?;
    Ok(prose_list)
}

fn render_prose_view(paths: &[String], prose_list: Vec<Prose>) -> String {
    maud! {
        fieldset #content class="border-y border-black dark:border-white-dark text-center max-w-prose" {
            legend class="mx-3 px-2" {
                h2 class="text-left text-purple font-mono dark:text-purple-dark" {
                    "<$> "
                    @for segment in paths {
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
                            span class="ml-1 text-sm text-gray dark:text-gray-dark font-mono p-0.5 outline rounded-sm outline-1 outline-gray/25 dark:outline-gray-dark/25 align-middle" { (prose.filename.clone()) }
                        }
                        div class="flex flex-wrap gap-1 p-1" {
                            @for tag in &prose.tags {
                                span class="px-1 py-0.5 bg-gray/25 dark:bg-gray-dark/25 text-sm" {
                                    "#" (tag)
                                }
                            }
                        }
                    }
                }
            }
        }
    }.render().into_inner()
}

fn render_error_view(paths: &[String], title: &str, error_message: &str) -> String {
    maud! {
        fieldset #content class="border-y border-black dark:border-white-dark text-center" {
            legend class="mx-3 px-2" {
                h2 class="text-left text-purple font-mono dark:text-purple-dark" {
                    "<$> "
                    @for segment in paths {
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
            p class="text-3xl" { (title) }
            p class="text-2xl mb-8" { "Essays, Poems, and others." }
            p class="text-red-600" { (error_message) }
        }
    }.render().into_inner()
}
