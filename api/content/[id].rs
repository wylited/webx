use std::collections::HashMap;

use hypertext::{maud, Renderable, html_elements, GlobalAttributes, Raw};
use vercel_runtime::{run, Body, Error, Request, Response, StatusCode};
use webx_api::{base, fetch, footer, header, Prose};
use url::Url;

#[tokio::main]
async fn main() -> Result<(), Error> {
    run(handler).await
}

pub async fn handler(req: Request) -> Result<Response<Body>, Error> {
    let mut url = Url::parse(&req.uri().to_string()).unwrap();
    let hash_query: HashMap<String, String> = url.query_pairs().into_owned().collect();
    let id = hash_query.get("id").unwrap();

    let paths = vec!["home".to_string(), "prose".to_string(), id.clone()];
    let song = "When I 226";

    url.set_path("/dist/prose.json");

    let json_url = url.as_str();

    let content = match fetch(&json_url) {
        Ok(json_content) => {
            match serde_json::from_str::<Vec<Prose>>(&json_content) {
                Ok(prose_list) => {
                    if let Some(prose) = prose_list.iter().find(|p| p.id == *id) {
                        maud! {
                            div class="border-y border-black dark:border-white-dark text-center py-4" {
                                h1 class="text-3xl mb-2" { (prose.title.clone()) }
                                p class="text-sm text-gray dark:text-gray-dark mb-4" {
                                    "Type: " (format!("{:?}", prose.prose_type))
                                        " | Source: " (prose.filename.clone())
                                }
                                div class="flex flex-wrap gap-1 mb-4 justify-center" {
                                    @for tag in &prose.tags {
                                        span class="px-1 py-0.5 bg-gray/25 dark:bg-gray-dark/25 text-sm" {
                                            "#"(tag)
                                        }
                                    }
                                }
                                div class="prose dark:prose-invert max-w-none text-left ml-2" {
                                    (Raw(&prose.html))
                                }
                            }
                        }.render().into_inner()
                    } else {
                        maud! {
                            div class="border-y border-black dark:border-white-dark text-center py-4" {
                                p class="text-red-600" { "Prose " (id) " not found" }
                            }
                        }.render().into_inner()
                    }
                },
                Err(e) => maud! {
                    div class="border-y border-black dark:border-white-dark text-center py-4" {
                        p class="text-red-600" { "Error parsing prose data!: " (e.to_string()) " from " (json_url) }
                    }
                }.render().into_inner()
            }
        },
        Err(e) => maud! {
            div class="border-y border-black dark:border-white-dark text-center py-4" {
                p class="text-red-600" { "Error fetching prose: " (e.to_string()) }
            }
        }.render().into_inner()
    };

    let page = base(&format!("{}{}{}", header(&paths, song), content, footer()));

    Ok(Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "text/html")
        .body(Body::Text(page))?
    )
}