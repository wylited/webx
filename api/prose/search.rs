use std::collections::HashMap;

use serde::Deserialize;
use webx_api::ExtraAttributes;
use hypertext::{html_elements, maud, Renderable, GlobalAttributes};
use url::Url;
use vercel_runtime::{run, Body, Error, Request, Response, StatusCode};
use webx_api::{fetch, Prose};

#[tokio::main]
async fn main() -> Result<(), Error> {
    run(handler).await
}

pub async fn handler(req: Request) -> Result<Response<Body>, Error> {
    let mut url = Url::parse(&req.uri().to_string()).unwrap();
    let hash_query: HashMap<String, String> = url.query_pairs().into_owned().collect();
    let query = hash_query.get("query").unwrap();
    println!("Query: {}", query);

    url.set_path("/dist/prose.json");

    let json_url = url.as_str();

    let content = match fetch(&json_url) {
        Ok(json_content) => {
            match serde_json::from_str::<Vec<Prose>>(&json_content) {
                Ok(prose_list) => {
                    let filtered_prose = Prose::search_collection(&prose_list, query);
                    maud! {
                        @for prose in filtered_prose.iter() {
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
                    }.render().into_inner()
                }
                Err(_) => maud! {
                    p class="text-red" {
                        "Error parsing prose JSON"
                    }
                }.render().into_inner()
            }
        }
        Err(_) => maud! {
                p class="text-red" {
                    "Error fetching prose JSON"
                }
            }.render().into_inner(),
    };

    Ok(Response::builder().status(StatusCode::OK).body(Body::Text(content))?)
}
