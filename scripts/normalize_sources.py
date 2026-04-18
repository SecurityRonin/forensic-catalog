#!/usr/bin/env python3
"""Normalize feed subscriptions, directory seeds, and manual additions.

Reads:
- archive/sources/dfir-feeds.opml
- archive/sources/catalog-directories.json
- archive/sources/manual-sources.json

Writes:
- archive/sources/source-inventory.json
- archive/sources/source-inventory.md

The generated inventory is the canonical machine-readable source map for the
repository. It does not replace per-artifact citations in the Rust catalog.
"""

from __future__ import annotations

import argparse
import json
import os
import re
import xml.etree.ElementTree as ET
from collections import Counter
from urllib.parse import urlsplit, urlunsplit


def normalize_url(url: str) -> str:
    if not url:
        return ""
    parts = urlsplit(url.strip())
    scheme = parts.scheme.lower() or "https"
    netloc = parts.netloc.lower()
    path = re.sub(r"/+$", "", parts.path or "")
    return urlunsplit((scheme, netloc, path, "", ""))


def slugify(value: str) -> str:
    slug = re.sub(r"[^a-z0-9]+", "-", value.lower()).strip("-")
    return slug or "source"


def load_json(path: str) -> list[dict]:
    with open(path, "r", encoding="utf-8") as handle:
        return json.load(handle)


def load_feeds(path: str) -> list[dict]:
    root = ET.parse(path).getroot()
    feeds: list[dict] = []
    for outline in root.findall(".//outline"):
        xml_url = outline.attrib.get("xmlUrl")
        if not xml_url:
            continue
        title = outline.attrib.get("title") or outline.attrib.get("text") or xml_url
        html_url = outline.attrib.get("htmlUrl", "")
        feeds.append(
            {
                "id": slugify(title),
                "title": title,
                "category": "feed",
                "kind": "rss-feed",
                "html_url": html_url,
                "feed_url": xml_url,
                "update_strategy": "rss",
                "notes": "Subscribed RSS/Atom feed from the maintained OPML manifest.",
                "sources": ["opml"],
            }
        )
    return feeds


def merge_entries(entries: list[dict]) -> list[dict]:
    merged: dict[str, dict] = {}
    for entry in entries:
        html_url = normalize_url(entry.get("html_url", ""))
        feed_url = normalize_url(entry.get("feed_url", ""))
        key = html_url or feed_url or entry["id"]
        current = merged.get(key)
        if current is None:
            current = {
                "id": entry["id"],
                "title": entry["title"],
                "category": entry["category"],
                "kind": entry["kind"],
                "html_url": entry.get("html_url", ""),
                "feed_url": entry.get("feed_url", ""),
                "update_strategy": entry.get("update_strategy", "manual-review"),
                "notes": entry.get("notes", ""),
                "sources": list(entry.get("sources", [])),
            }
            merged[key] = current
            continue

        current["sources"] = sorted(set(current["sources"]) | set(entry.get("sources", [])))
        if not current.get("feed_url") and entry.get("feed_url"):
            current["feed_url"] = entry["feed_url"]
        if not current.get("html_url") and entry.get("html_url"):
            current["html_url"] = entry["html_url"]
        if current.get("update_strategy") != "rss" and entry.get("update_strategy") == "rss":
            current["update_strategy"] = "rss"
        if len(entry.get("notes", "")) > len(current.get("notes", "")):
            current["notes"] = entry["notes"]
        if current["category"] == "feed" and entry["category"] != "feed":
            current["category"] = entry["category"]
        if current["kind"] == "rss-feed" and entry["kind"] != "rss-feed":
            current["kind"] = entry["kind"]

    return sorted(merged.values(), key=lambda item: (item["category"], item["title"].lower()))


def write_json(path: str, payload: dict) -> None:
    with open(path, "w", encoding="utf-8") as handle:
        json.dump(payload, handle, ensure_ascii=False, indent=2)
        handle.write("\n")


def write_markdown(path: str, inventory: list[dict], inputs: dict[str, str]) -> None:
    counts = Counter(item["category"] for item in inventory)
    lines = [
        "# Canonical Source Inventory",
        "",
        "Generated from:",
        f"- feeds: `{inputs['feeds']}`",
        f"- directories: `{inputs['directories']}`",
        f"- manual additions: `{inputs['manual']}`",
        "",
        "Category counts:",
    ]
    for category, count in sorted(counts.items()):
        lines.append(f"- {category}: {count}")
    lines.append("")

    grouped: dict[str, list[dict]] = {}
    for item in inventory:
        grouped.setdefault(item["category"], []).append(item)

    for category in sorted(grouped):
        lines.append(f"## {category}")
        lines.append("")
        for item in grouped[category]:
            lines.append(f"- {item['title']}: {item['html_url'] or item['feed_url']}")
            lines.append(f"  update: `{item['update_strategy']}`; kind: `{item['kind']}`; sources: `{', '.join(item['sources'])}`")
        lines.append("")

    with open(path, "w", encoding="utf-8") as handle:
        handle.write("\n".join(lines).rstrip() + "\n")


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--feeds", default="archive/sources/dfir-feeds.opml")
    parser.add_argument("--directories", default="archive/sources/catalog-directories.json")
    parser.add_argument("--manual", default="archive/sources/manual-sources.json")
    parser.add_argument("--output-json", default="archive/sources/source-inventory.json")
    parser.add_argument("--output-md", default="archive/sources/source-inventory.md")
    args = parser.parse_args()

    feeds = load_feeds(args.feeds)
    directories = load_json(args.directories)
    manual = load_json(args.manual)

    for item in directories:
        item.setdefault("sources", ["catalog-directory"])
    for item in manual:
        item.setdefault("sources", ["manual"])

    inventory = merge_entries(feeds + directories + manual)
    payload = {
        "generated_from": {
            "feeds": args.feeds,
            "directories": args.directories,
            "manual": args.manual,
        },
        "total_sources": len(inventory),
        "sources": inventory,
    }

    os.makedirs(os.path.dirname(args.output_json), exist_ok=True)
    write_json(args.output_json, payload)
    write_markdown(
        args.output_md,
        inventory,
        {"feeds": args.feeds, "directories": args.directories, "manual": args.manual},
    )
    print(f"normalized {len(inventory)} sources")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
