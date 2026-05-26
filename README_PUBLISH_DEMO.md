# Publish Demo (Prototype 001)

This repo now includes a tiny **publish lane prototype** in `publish-demo/`.

## What this proves

- A local source packet (`publish-demo/source/page.json`) can be validated.
- A local render command can generate a static page (`publish-demo/dist/index.html`).
- The normalized packet can be emitted for publishing (`publish-demo/dist/page.json`).
- A manifest can capture provenance (`publish-demo/publish-manifest.json`).
- Only the generated `publish-demo/dist` directory needs to be deployed to Cloudflare Pages.

## Files in the prototype

- `publish-demo/source/page.json` - editable source packet for one approved page.
- `publish-demo/render.py` - local render + validation script.
- `publish-demo/dist/index.html` - generated static output.
- `publish-demo/dist/page.json` - generated normalized JSON packet.
- `publish-demo/publish-manifest.json` - generated manifest with timestamp and outputs.

## Local render command

From repo root:

```bash
python3 publish-demo/render.py
```

The command exits non-zero if required fields are missing from `publish-demo/source/page.json`.

Required fields:

- `id`
- `title`
- `headline`
- `body`

## Deploy to Cloudflare Pages

Use any Cloudflare Pages flow that publishes a static directory. The safe path is:

1. Render locally first:

```bash
python3 publish-demo/render.py
```

2. Publish only `publish-demo/dist`.

With Wrangler CLI (no secrets committed to repo):

```bash
npx wrangler pages deploy publish-demo/dist --project-name <your-pages-project>
```

You can also use the Cloudflare dashboard and set the upload directory to `publish-demo/dist`.

## Intentionally not included yet

- No migration of existing app/runtime.
- No AI calls.
- No database.
- No Cloudflare Tunnel.
- No secret material in this repository.
- No paid services required.

This is a scout lane / spike to validate the publish path only.
