#!/usr/bin/env python3
"""Poll subscribed feeds and write a compact snapshot/report.

Reads `archive/sources/dfir-feeds.opml`, fetches feed entries for any outline
that has an `xmlUrl`, and writes:

- `archive/sources/feed-state.json`
- `archive/sources/feed-report.md`

Designed for scheduled GitHub Actions runs. No third-party dependencies.
"""

from __future__ import annotations

import argparse
import email.utils
import json
import os
import time
import urllib.error
import urllib.request
import xml.etree.ElementTree as ET
from dataclasses import asdict, dataclass
from typing import Iterable


USER_AGENT = "forensic-catalog-feed-watcher/0.1 (+https://github.com/SecurityRonin/forensic-catalog)"
ATOM_NS = {"atom": "http://www.w3.org/2005/Atom"}


@dataclass
class FeedEntry:
    title: str
    url: str
    published: str


@dataclass
class FeedSnapshot:
    title: str
    html_url: str
    xml_url: str
    checked_at: str
    entries: list[FeedEntry]
    error: str | None


def fetch(url: str) -> bytes:
    request = urllib.request.Request(url, headers={"User-Agent": USER_AGENT})
    try:
        with urllib.request.urlopen(request, timeout=30) as response:
            return response.read()
    except urllib.error.HTTPError as exc:
        raise RuntimeError(f"HTTP {exc.code}") from exc
    except urllib.error.URLError as exc:
        raise RuntimeError(str(exc.reason)) from exc


def iter_feed_outlines(opml_path: str) -> Iterable[dict[str, str]]:
    root = ET.parse(opml_path).getroot()
    for outline in root.findall(".//outline"):
        xml_url = outline.attrib.get("xmlUrl")
        if not xml_url:
            continue
        yield {
            "title": outline.attrib.get("title") or outline.attrib.get("text") or xml_url,
            "html_url": outline.attrib.get("htmlUrl", ""),
            "xml_url": xml_url,
        }


def parse_isoish(value: str) -> str:
    value = (value or "").strip()
    if not value:
        return ""
    parsed = email.utils.parsedate_to_datetime(value)
    if parsed is not None:
        return parsed.isoformat()
    return value


def parse_atom(root: ET.Element) -> list[FeedEntry]:
    entries: list[FeedEntry] = []
    for entry in root.findall("atom:entry", ATOM_NS):
        title = entry.findtext("atom:title", default="", namespaces=ATOM_NS).strip()
        published = (
            entry.findtext("atom:updated", default="", namespaces=ATOM_NS)
            or entry.findtext("atom:published", default="", namespaces=ATOM_NS)
        )
        url = ""
        for link in entry.findall("atom:link", ATOM_NS):
            rel = link.attrib.get("rel", "alternate")
            if rel == "alternate":
                url = link.attrib.get("href", "")
                break
        entries.append(FeedEntry(title=title, url=url, published=parse_isoish(published)))
    return entries


def parse_rss(root: ET.Element) -> list[FeedEntry]:
    entries: list[FeedEntry] = []
    for item in root.findall(".//item"):
        title = (item.findtext("title") or "").strip()
        url = (item.findtext("link") or "").strip()
        published = parse_isoish(item.findtext("pubDate") or "")
        entries.append(FeedEntry(title=title, url=url, published=published))
    return entries


def parse_feed(payload: bytes) -> list[FeedEntry]:
    root = ET.fromstring(payload)
    tag = root.tag.lower()
    if tag.endswith("feed"):
        return parse_atom(root)
    return parse_rss(root)


def now_iso() -> str:
    return time.strftime("%Y-%m-%dT%H:%M:%SZ", time.gmtime())


def load_previous(path: str) -> dict[str, dict]:
    if not os.path.exists(path):
        return {}
    with open(path, "r", encoding="utf-8") as handle:
        return json.load(handle)


def write_state(path: str, snapshots: list[FeedSnapshot]) -> None:
    with open(path, "w", encoding="utf-8") as handle:
        json.dump({snap.xml_url: asdict(snap) for snap in snapshots}, handle, ensure_ascii=False, indent=2)
        handle.write("\n")


def write_report(path: str, snapshots: list[FeedSnapshot], previous: dict[str, dict]) -> int:
    changes = 0
    lines = [
        "# Feed Update Report",
        "",
        f"Generated: {now_iso()}",
        "",
    ]

    for snap in snapshots:
        lines.append(f"## {snap.title}")
        lines.append("")
        lines.append(f"- Site: {snap.html_url or 'unknown'}")
        lines.append(f"- Feed: {snap.xml_url}")
        if snap.error:
            lines.append(f"- Status: error: {snap.error}")
            lines.append("")
            continue
        previous_entries = previous.get(snap.xml_url, {}).get("entries", [])
        previous_urls = {entry.get('url', '') for entry in previous_entries}
        new_entries = [entry for entry in snap.entries if entry.url and entry.url not in previous_urls]
        lines.append(f"- Entries checked: {len(snap.entries)}")
        lines.append(f"- New since last snapshot: {len(new_entries)}")
        lines.append("")
        for entry in new_entries[:10]:
            changes += 1
            lines.append(f"- {entry.published or 'unknown date'} — [{entry.title}]({entry.url})")
        if not new_entries:
            lines.append("- No new entries detected")
        lines.append("")

    with open(path, "w", encoding="utf-8") as handle:
        handle.write("\n".join(lines).rstrip() + "\n")
    return changes


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--opml", default="archive/sources/dfir-feeds.opml")
    parser.add_argument("--state", default="archive/sources/feed-state.json")
    parser.add_argument("--report", default="archive/sources/feed-report.md")
    parser.add_argument("--limit", type=int, default=10, help="max entries retained per feed")
    args = parser.parse_args()

    previous = load_previous(args.state)
    snapshots: list[FeedSnapshot] = []
    for outline in iter_feed_outlines(args.opml):
        error = None
        entries: list[FeedEntry] = []
        try:
            payload = fetch(outline["xml_url"])
            entries = parse_feed(payload)[: args.limit]
        except Exception as exc:
            error = str(exc)
        snapshots.append(
            FeedSnapshot(
                title=outline["title"],
                html_url=outline["html_url"],
                xml_url=outline["xml_url"],
                checked_at=now_iso(),
                entries=entries,
                error=error,
            )
        )

    os.makedirs(os.path.dirname(args.state), exist_ok=True)
    write_state(args.state, snapshots)
    changes = write_report(args.report, snapshots, previous)
    print(f"checked {len(snapshots)} feeds; detected {changes} new entries")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
