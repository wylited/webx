use hypertext::{html_elements, maud, GlobalAttributes, Raw, Renderable};
use url::Url;
use vercel_runtime::{run, Body, Error, Request, Response, StatusCode};
use webx_api::{fetch, Project};

#[tokio::main]
async fn main() -> Result<(), Error> {
    run(handler).await
}

pub async fn handler(req: Request) -> Result<Response<Body>, Error> {
    let paths = vec!["home".to_string(), "projects".to_string()];
    let mut url = Url::parse(&req.uri().to_string()).unwrap();

    url.set_path("/dist/projects.json");

    let json_url = url.as_str();

    // Fetch and parse the JSON data
    let content = match fetch_projects(json_url).await {
        Ok(projects) => render_projects_view(&paths, projects),
        Err(err) => render_error_view(&paths, "Error fetching projects", &err.to_string()),
    };

    Ok(Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "text/html")
        .body(Body::Text(content))?)
}

// Fetch the projects JSON data
async fn fetch_projects(json_url: &str) -> Result<Vec<Project>, Error> {
    let response = fetch(json_url)?;
    let projects: Vec<Project> = serde_json::from_str(&response)?;
    Ok(projects)
}

// Render the projects view
fn render_projects_view(paths: &[String], projects: Vec<Project>) -> String {
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
            p class="text-3xl py-2" { "My Favorite Projects" }
            div class="projects-container m-4 rounded" {
                @for project in projects.iter() {
                    div class="project-card border border-gray dark:border-gray-dark rounded p-4 mb-4 text-left" {
                        h3 class="text-2xl font-bold mb-2" { (project.title.clone()) }
                        div class="tags flex flex-wrap gap-2 mb-2" {
                            @for tag in &project.tags {
                                span class="px-1 py-0.5 bg-gray/25 dark:bg-gray-dark/25 text-sm" {
                                    "#" (tag)
                                }
                            }
                        }
                        div class="description mb-2 text-gray-800 dark:text-gray-200" {
                            (Raw(&project.description))
                        }
                        div class="links flex gap-2" {

                            @for (key, value) in &project.links {
                                a
                                    class="px-4 py-2 rounded border border-gray dark:border-gray-dark hover:bg-gray hover:text-white hover:dark:text-white hover:dark:bg-gray-dark transition"
                                    href=(value)
                                    target="_blank"
                                {
                                    (capitalize_first_letter(key))
                                }
                            }
                        }
                    }
                }
            }
            p class="text-xl mt-4" { "Checkout my "
                                       a href="https://github.com/wylited" {
                                           "GitHub"
                                       }
                                       " for more projects!" }
        }
    }.render().into_inner()
}

// Render a generic error view
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
            p class="text-2xl mb-8" { "An error occurred while fetching data. Checkout my github instead?" }
            a href="https://github.com/wylited" {
                "GitHub"
            }
            p class="text-red-600" { (error_message) }
        }
    }.render().into_inner()
}

// Helper function to capitalize the first letter of a string
fn capitalize_first_letter(s: &str) -> String {
    let mut chars = s.chars();
    match chars.next() {
        Some(c) => c.to_uppercase().collect::<String>() + chars.as_str(),
        None => String::new(),
    }
}
