#![allow(non_upper_case_globals)]
use anyhow::Result;
use hypertext::{html_elements, maud, Attribute, GlobalAttributes, Raw, Renderable};
use orgize::Org;
use serde::{Deserialize, Serialize};
use std::{collections::HashSet, fs, path::PathBuf};

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
    const hx_target: Attribute = Attribute;
    const hx_swap: Attribute = Attribute;
    const hx_push_url: Attribute = Attribute;
    const onclick: Attribute = Attribute;
}

impl<T: GlobalAttributes> ExtraAttributes for T {}

pub fn header(path_segments: &[String], current_song: &str) -> String {
    maud! {
        div class="flex justify-between items-center mb-1" {
            div class="content-center" {
                h2 class="text-left text-purple font-mono dark:text-purple-dark" {
                    "<$> "
                    @for segment in path_segments {
                        a
                            class="nav"
                            href={ "/" (segment) }
                        {
                            (segment) "/"
                        }
                    }
                }
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
                button #themeBtn class="link" onclick="toggleTheme()" {"☀"}
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

pub fn parse_file(path: &PathBuf) -> Result<Prose> {
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

pub fn base(content: &str) -> String {
    maud! {
        !DOCTYPE
        html lang="en" class="dark" {
            head {
                meta charset="UTF-8";
                meta name="viewport" content="width=device-width, initial-scale=1.0";
                title { "wypage" }
                link
                    rel="preload"
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
                script src="/dist/htmx.min.js" {}
                script src="/scripts.js" {}
            }
            body class="bg-white dark:bg-black-dark p-10 max-w-full" {
                div class="flex flex-col items-center justify-center min-h-screen w-full max-w-full"{
                    div class="max-w-full" {
                        (Raw(content))
                    }
                }
            }
        }
    }
    .render()
    .into_inner()
}
