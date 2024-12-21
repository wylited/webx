# [My personal web presence](https://wylited.vercel.app)

A highly opinonated, minimalistic, and blazingly fast personal website with a content backend written almost completely in Rust.

## Built With
- [htmx](https://htmx.org)
- [htmz](https://leanrada.com/htmz/)
- [tailwindcss](https://tailwindcss.com/)
- [hypertext](https://github.com/vidhanio/hypertext)
- [orgize](https://github.com/poiscript/orgize)
- [anyhow](https://github.com/dtolnay/anyhow)

htmx, htmz, htmy when?

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

