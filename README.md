# [My personal web presence](https://wylited.vercel.app)

## Built With
- htmx
- tailwindcss
- hypertext-rs
- orgize

## Deploying

Make sure you have the development packages installed

``` shell
pnpm install
```

You also need linux musl
``` shell
rustup target add x86_64-unknown-linux-musl
```

Then you can prebuild the site and deploy it via Vercel CLI

``` shell
vercel build && vercel deploy --prebuilt
```

