# [My personal web presence](https://wylited.vercel.app)

A highly opinonated, minimalistic, and blazingly fast personal website with a content backend written almost completely in Rust.

## Built With
- [htmx](https://htmx.org) AJAX but better
- [htmz](https://leanrada.com/htmz/) Loading HTML directly into the DOM
- [tailwindcss](https://tailwindcss.com/) Styling in tag classes
- [hypertext](https://github.com/vidhanio/hypertext) generating HTML in Rust
- [orgize](https://github.com/poiscript/orgize) parsing org-mode files in Rust
- [anyhow](https://github.com/dtolnay/anyhow) error handling

So we got htmx and htmz; htmy when?

## Updating Prose

After cloning the repo and updating `src/prose` with more prose

``` shell
cargo run --package webx_api --bin processor -- 
```

## Adding a new page

Create a new api binary in `Cargo.toml`

``` toml
[[bin]]
name = "name"
path = "api/name.rs"
```

This will map the endpoint to `/api/name` 

## Local Development

First compile all the local pages (rust binaries).

``` shell
cargo build --release
```

And then start a dev server

``` shell
vercel dev
```

## Deploying prebuilt

``` shell
vercel build && vercel deploy --prebuilt 
```

