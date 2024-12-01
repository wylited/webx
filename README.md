# [My personal web presence](https://wylited.vercel.app)

## Built With
- [htmx](https://htmx.org/)
- [tailwindcss]
- hypertext
- orgize

## Updating Prose

After cloning the repo and updating `src/prose`

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

## Deploying prebuilt

``` shell
vercel build && vercel deploy --prebuilt 
```

