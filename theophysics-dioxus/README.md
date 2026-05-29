# Theophysics Dioxus Workspace (Phase 1 scaffold)

This scaffold starts Phase 1 with a shared component crate and one working web app prototype.

## What is included
- `crates/common`: shared types + starter components (`ArticleView`, `Sidebar`, `TabSystem`, `AudioPlayer`).
- `crates/faiththru`: Phase 1 article reader prototype loading 3 article JSON files.
- `crates/dashboard`, `crates/comms`: placeholders for Phase 2/3.
- `assets/tailwind.css`: placeholder stylesheet slot for Tailwind build output.

## Run locally
```bash
cd theophysics-dioxus
cargo run -p faiththru
```

If you have Dioxus CLI installed:
```bash
cd theophysics-dioxus
# choose crates/faiththru as app dir for dx serve in your local setup
dx serve
```

## Deployment direction (Cloudflare Pages)
1. Build web assets for the Phase 1 app.
2. Deploy generated static directory to Cloudflare Pages.
3. Keep media URLs pointed at Cloudflare R2.

## Early technical callout
- For equations: KaTeX is usually easier than MathJax in Rust/WASM UI stacks for predictable client rendering.
