#!/usr/bin/env python3
from __future__ import annotations

import html
import json
import sys
from datetime import datetime, timezone
from pathlib import Path

ROOT = Path(__file__).resolve().parent
SOURCE = ROOT / "source" / "page.json"
DIST = ROOT / "dist"
OUT_HTML = DIST / "index.html"
OUT_JSON = DIST / "page.json"
MANIFEST = ROOT / "publish-manifest.json"

REQUIRED_FIELDS = ["id", "title", "headline", "body"]


def fail(message: str) -> None:
    print(f"render error: {message}", file=sys.stderr)
    raise SystemExit(1)


def load_source() -> dict:
    if not SOURCE.exists():
        fail(f"missing source file: {SOURCE}")
    try:
        data = json.loads(SOURCE.read_text(encoding="utf-8"))
    except json.JSONDecodeError as exc:
        fail(f"invalid JSON in {SOURCE}: {exc}")

    if not isinstance(data, dict):
        fail("source JSON must be an object")

    missing = [field for field in REQUIRED_FIELDS if not str(data.get(field, "")).strip()]
    if missing:
        fail(f"missing required field(s): {', '.join(missing)}")

    normalized = {
        "id": str(data["id"]).strip(),
        "title": str(data["title"]).strip(),
        "headline": str(data["headline"]).strip(),
        "body": str(data["body"]).strip(),
    }

    if "updated_at" in data and str(data["updated_at"]).strip():
        normalized["updated_at"] = str(data["updated_at"]).strip()

    return normalized


def render_html(page: dict) -> str:
    title = html.escape(page["title"])
    headline = html.escape(page["headline"])
    body = html.escape(page["body"])
    page_id = html.escape(page["id"])

    return f"""<!doctype html>
<html lang=\"en\">
  <head>
    <meta charset=\"utf-8\" />
    <meta name=\"viewport\" content=\"width=device-width, initial-scale=1\" />
    <title>{title}</title>
  </head>
  <body>
    <main>
      <h1>{headline}</h1>
      <p>{body}</p>
      <small>Page ID: {page_id}</small>
    </main>
  </body>
</html>
"""


def write_outputs(page: dict) -> None:
    DIST.mkdir(parents=True, exist_ok=True)
    OUT_HTML.write_text(render_html(page), encoding="utf-8")
    OUT_JSON.write_text(json.dumps(page, indent=2) + "\n", encoding="utf-8")

    manifest = {
        "generated_at": datetime.now(timezone.utc).isoformat(),
        "source_file": str(SOURCE.relative_to(ROOT.parent)),
        "output_files": [
            str(OUT_HTML.relative_to(ROOT.parent)),
            str(OUT_JSON.relative_to(ROOT.parent)),
        ],
        "page_id": page["id"],
    }
    MANIFEST.write_text(json.dumps(manifest, indent=2) + "\n", encoding="utf-8")


def main() -> None:
    page = load_source()
    write_outputs(page)
    print(f"rendered {OUT_HTML} and {OUT_JSON}")


if __name__ == "__main__":
    main()
