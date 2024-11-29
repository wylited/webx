#![allow(non_upper_case_globals)]
use hypertext::{html_elements, maud, Attribute, GlobalAttributes, Renderable};

pub trait ExtraAttributes: GlobalAttributes {
    const hx_get: Attribute = Attribute;
    const hx_target: Attribute = Attribute;
    const onclick: Attribute = Attribute;
}

impl<T: GlobalAttributes> ExtraAttributes for T {}

pub fn header(path_segments: &[String], current_song: &str) -> String {
    maud! {
        div class="flex justify-between items-center mb-1" {
            div class="content-center" {
                h2 class="text-left text-purple font-mono dark:text-purple-dark" {
                    "$> "
                    @for segment in path_segments {
                        button
                            class="nav"
                            hx-get={ "/api/" (segment) }
                            hx-target="#content"
                        {
                            (segment) "/"
                        }
                    }
                }
            }
            div class="text-center font-mono" {
                audio #audio src="dist/wheni226.mp3" loop {}
                p #song {
                    button #audioBtn onclick="togglePlay()" {""}
                    span class="pl-4" { (current_song) } " "
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
                        a href="https://github.com/wylited/webx" {
                            "webx"
                        }
                }
            }
            div class="text-center font-mono" {
                button #themeBtn onclick="toggleTheme()" {""}
                span #time class="pl-4" {
                    " "
                }
            }
        }
    }
    .render()
    .into_inner()
}
