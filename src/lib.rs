#![allow(non_upper_case_globals)]
use anyhow::Result;
use fuzzy_matcher::skim::SkimMatcherV2;
use fuzzy_matcher::FuzzyMatcher;
use hypertext::{html_elements, maud, Attribute, GlobalAttributes, Raw, Renderable};
use orgize::Org;
use serde::{Deserialize, Serialize};
use std::{
    collections::{HashMap, HashSet},
    fs,
    path::PathBuf,
};

pub fn fetch(url: &str) -> anyhow::Result<String> {
    let url = if url.contains("localhost") && url.starts_with("https://") {
        url.replace("https://", "http://")
    } else {
        url.to_string()
    };

    let body = ureq::get(&url).call()?.into_string()?;
    Ok(body)
}

pub trait ExtraAttributes: GlobalAttributes {
    const hx_get: Attribute = Attribute;
    const hx_post: Attribute = Attribute;
    const hx_trigger: Attribute = Attribute;
    const hx_indicator: Attribute = Attribute;
    const hx_target: Attribute = Attribute;
    const hx_swap: Attribute = Attribute;
    const hx_push_url: Attribute = Attribute;
    const onclick: Attribute = Attribute;
    const onload: Attribute = Attribute;
}

impl<T: GlobalAttributes> ExtraAttributes for T {}

pub fn header(current_song: &str) -> String {
    maud! {
        div class="flex justify-between items-center" {
            div class="content-center" {
                // h2 class="text-left text-purple font-mono dark:text-purple-dark" {
                //     "<$> "
                //     @for segment in path_segments {
                //         a
                //             class="nav"
                //             href={ "/" (segment) }
                //         {
                //             (segment) "/"
               //         }
                //     }
                // }
            }
            div class="text-center font-mono" {
                audio #audio src="dist/wheni226.mp3" loop {}
                p #song {
                    button #audioBtn class="link" onclick="togglePlay()" {"|>"}
                    span class="pl-2" { (current_song) } " "
                    span #duration { "00:00" }
                }
            }
        }
    }
    .render()
    .into_inner()
}

pub fn home(paths: Vec<String>) -> String {
    maud! {
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
                    "Math AA HL, Physics HL, Comp Sci HL, Econ HL"
                }
            }
            p class="mb-2 relative inline-block after:content-[''] after:absolute after:bottom-0 after:left-0 after:w-[49%] after:h-[4px] after:bg-orange dark:after:bg-orange-dark" {
                "Now Reading: Writing to Learn, 49%"
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
    .into_inner()
}

pub fn footer() -> String {
    maud! {
        div class="flex justify-between items-center mt-1" {
            div class="content-center" {
                h2 class="text-left text-purple font-mono dark:text-purple-dark" {
                    "© 2024 "
                        a class="link" href="https://github.com/wylited/webx" {
                            "webx"
                        }
                }
            }
            div class="text-center font-mono" {
                button #themeBtn class="link text-xl" onclick="toggleTheme()" {"☀"}
                span #time class="pl-3" {
                    " "
                }
            }
        }
    }
    .render()
    .into_inner()
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum ProseTypes {
    Essay, // Essaying
    Poem,  // Specfically for poems
    Prose, // General prose
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Prose {
    pub id: String,
    pub prose_type: ProseTypes,
    pub title: String,
    pub filename: String,
    pub org: String,
    pub tags: Vec<String>,
    pub html: String,
}

pub fn parse_prose(path: &PathBuf) -> Result<Prose> {
    let org = fs::read_to_string(path)?; // Read the file to a string

    // Extract the filename from the path
    let filename = path
        .file_name()
        .ok_or_else(|| anyhow::anyhow!("Invalid filename"))?
        .to_string_lossy()
        .into_owned();

    // Parse the org file
    let content = Org::parse(&org);

    // Extract the properties from the org file using orgize
    let properties = content
        .document()
        .properties()
        .ok_or_else(|| anyhow::anyhow!("No properties found"))?
        .to_hash_map();

    // Extract the ID from the properties
    let id = properties
        .get("ID")
        .ok_or_else(|| anyhow::anyhow!("No ID property found"))?
        .to_string();

    // Extract the tags from the properties
    let tags: HashSet<String> = content
        .document()
        .keywords()
        .find(|keyword| keyword.key() == "filetags")
        .map(|keyword| {
            keyword
                .value()
                .split(':')
                .map(String::from)
                .filter(|tag| !tag.trim().is_empty()) // Filter out empty strings and those with only spaces
                .collect()
        })
        .unwrap_or_else(HashSet::new);

    // check if any of the tags are blog or poem and set the prose type accordingly
    let mut prosetype = ProseTypes::Prose;

    if tags.contains("essay") {
        prosetype = ProseTypes::Essay;
    } else if tags.contains("poem") {
        prosetype = ProseTypes::Poem;
    }

    Ok(Prose {
        id,
        prose_type: prosetype,
        title: content.title().unwrap_or_else(|| "Untitled".to_string()),
        filename,
        org,
        tags: tags.into_iter().collect(),
        html: content.to_html(),
    })
}

pub fn parse_project(path: &PathBuf) -> Result<Project> {
    let org = fs::read_to_string(path)?; // Read the file to a string

    // Parse the org file
    let content = Org::parse(&org);

    // Extract the properties from the org file using orgize
    let properties = content
        .document()
        .properties()
        .ok_or_else(|| anyhow::anyhow!("No properties found"))?
        .to_hash_map();

    // Extract the ID from the properties
    let id = properties
        .get("ID")
        .ok_or_else(|| anyhow::anyhow!("No ID property found"))?
        .to_string();

    let links: HashMap<String, String> = properties
        .iter()
        .filter(|(key, _)| key.ends_with("LINK"))
        .map(|(key, value)| {
            let link = key.replace("LINK", "").to_lowercase();
            (link, value.to_string())
        })
        .collect();

    // Extract the tags from the properties
    let tags: HashSet<String> = content
        .document()
        .keywords()
        .find(|keyword| keyword.key() == "filetags")
        .map(|keyword| {
            keyword
                .value()
                .split(':')
                .map(String::from)
                .filter(|tag| !tag.trim().is_empty()) // Filter out empty strings and those with only spaces
                .collect()
        })
        .unwrap_or_else(HashSet::new);

    Ok(Project {
        id,
        title: content.title().unwrap_or_else(|| "Untitled".to_string()),
        description: content.to_html(),
        tags: tags.into_iter().collect(),
        links,
    })
}

pub fn base(content: &str) -> String {
    maud! {
        !DOCTYPE
        html lang="en" class="" {
            head {
                meta charset="UTF-8";
                meta name="viewport" content="width=device-width, initial-scale=1.0";
                title { "wypage" }
                link
                    rel="preconnect"
                    href="https://www.programmingfonts.org/fonts/resources/firacode/firacode.woff2"
                    as="font"
                    type="font/woff2";
                link
                    rel="preconnect"
                    href="https://fonts.googleapis.com";
                link
                    rel="preconnect"
                    href="https://fonts.gstatic.com"
                    crossorigin;
                link
                    href="https://fonts.googleapis.com/css2?family=Lexend:wght@100..900&display=swap"
                    rel="stylesheet";
                link
                    href="/dist/style.css"
                    rel="stylesheet";
                script src="/scripts.js" {}
            }
            body class="bg-white dark:bg-black-dark p-10 max-w-full max-h-screen transition-colors duration-300 ease-in-out" {
                iframe hidden name="htmz" onload="handleHtmzTransition(this)" {}
                div class="flex flex-col items-center justify-center min-h-screen w-full max-w-full"{
                    div class="max-w-full w-full md:w-3/4 lg:w-1/2" {
                        (Raw(content))
                    }
                }
            }
        }
    }
    .render()
    .into_inner()
}

impl Prose {
    pub fn matches_fuzzy(&self, query: &str) -> Option<i64> {
        let matcher = SkimMatcherV2::default();

        // Create a combined string of searchable fields
        let searchable_text = format!(
            "{} {} {}",
            self.title,
            // self.org, to decide
            self.tags.join(" "),
            self.filename
        );

        // Return the match score if there's a match
        matcher.fuzzy_match(&searchable_text, query)
    }

    pub fn search_collection(prose_collection: &[Prose], query: &str) -> Vec<Prose> {
        let mut matches: Vec<(i64, Prose)> = prose_collection
            .iter()
            .filter_map(|prose| {
                prose
                    .matches_fuzzy(query)
                    .map(|score| (score, prose.clone()))
            })
            .collect();

        matches.sort_by(|a, b| b.0.cmp(&a.0));
        matches.into_iter().map(|(_, prose)| prose).collect()
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Project {
    pub id: String,
    pub title: String,
    pub description: String,
    pub tags: Vec<String>,
    pub links: HashMap<String, String>,
}
