use hypertext::{html_elements, maud, Renderable, GlobalAttributes};
use vercel_runtime::{run, Body, Error, Request, Response, StatusCode};
use webx_api::{fetch, footer, header, Prose};

#[tokio::main]
async fn main() -> Result<(), Error> {
    run(handler).await
}

pub async fn handler(req: Request) -> Result<Response<Body>, Error> {
    let paths = vec!["home".to_string(), "prose".to_string()];
    let song = "When I 226";
    let uri = req.uri().to_string();

    let base_uri = uri.strip_suffix("/api/prose")
        .unwrap_or(&uri);

    let json_url = format!("{}/dist/prose.json", base_uri);

    let content = match fetch(&json_url) {
        Ok(json_content) => {
            match serde_json::from_str::<Vec<Prose>>(&json_content) {
                Ok(prose_list) => maud! {
                    div class="border-y border-black dark:border-white-dark text-center py-4" {
                        p class="text-3xl" { "Prose" }
                        p class="text-2xl mb-2" { "Essays, Poems, and more." }
                        div class="flex flex-col" {
                            @for prose in prose_list.iter() {
                                div class="w-16 my-4 border-t border-gray/20 dark:border-gray-dark/20" {}
                                div class="w-full max-w-full text-left ml-4" {
                                    h2 class="text-2xl mb-1" { (prose.title.clone()) " " } span class="text-sm text-gray dark:text-gray-dark font-mono mb-1" { (prose.filename.clone()) }
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
                    div class="border-y border-black dark:border-white-dark text-center py-4" {
                        p class="text-3xl" { "Prose" }
                        p class="text-2xl mb-8" { "Essays, Poems, and others." }
                        p class="text-red-600" { "Error parsing prose data: " (e.to_string()) }
                    }
                }.render().into_inner()
            }
        },
        Err(e) => maud! {
            div class="border-y border-black dark:border-white-dark text-center py-4" {
                p class="text-3xl" { "Prose" }
                p class="text-2xl mb-8" { "Essays, Poems, and others." }
                p class="text-red-600" { "Error fetching prose: " (e.to_string()) }
            }
        }.render().into_inner()
    };

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
