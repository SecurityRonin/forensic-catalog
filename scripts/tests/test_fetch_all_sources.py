"""
Tests for scripts/fetch_all_sources.py

Tests cover the pure-logic functions (no real HTTP):
  - parse_blogger_feed(xml_text)       → list[tuple[str,str,str]]
  - parse_wordpress_posts(json_text)   → list[tuple[str,str,str]]
  - parse_atom_feed(xml_text)          → list[tuple[str,str,str]]
  - parse_github_commits(json_text)    → list[tuple[str,str,str]]
  - load_seen_urls(pending_path) → set[str]  (dedup against pending-review.md)
  - classify_blog_source(html_url)     → str  ("blogger"|"wordpress"|"github"|"unknown")
  - rescan_reviewed_entries(path)      → int  (rewrites [x] → [ ])
"""

import json
import os
import sys
import textwrap
import unittest

# fetch_all_sources.py lives one level up from this test file
sys.path.insert(0, os.path.join(os.path.dirname(__file__), ".."))

import fetch_all_sources as ba  # noqa: E402  (module under test)


BLOGGER_FEED_XML = textwrap.dedent("""\
<?xml version="1.0" encoding="UTF-8"?>
<feed xmlns="http://www.w3.org/2005/Atom">
  <entry>
    <title>ShimCache and AppCompatCache</title>
    <link rel="alternate" href="https://windowsir.blogspot.com/2024/01/shimcache.html"/>
    <published>2024-01-15T10:00:00Z</published>
  </entry>
  <entry>
    <title>UserAssist Deep Dive</title>
    <link rel="alternate" href="https://windowsir.blogspot.com/2023/12/userassist.html"/>
    <published>2023-12-01T10:00:00Z</published>
  </entry>
</feed>
""")

WORDPRESS_API_JSON = json.dumps([
    {
        "title": {"rendered": "Prefetch Analysis"},
        "link": "https://dfir.blog/prefetch-analysis/",
        "date": "2024-02-10T09:00:00",
    },
    {
        "title": {"rendered": "SRUM Database"},
        "link": "https://dfir.blog/srum-database/",
        "date": "2024-01-20T09:00:00",
    },
])

GITHUB_ATOM_XML = textwrap.dedent("""\
<?xml version="1.0" encoding="UTF-8"?>
<feed xmlns="http://www.w3.org/2005/Atom">
  <entry>
    <title>Create certutil.yml</title>
    <link rel="alternate" href="https://github.com/LOLBAS-Project/LOLBAS/commit/abc123"/>
    <updated>2024-03-01T12:00:00Z</updated>
  </entry>
</feed>
""")

GITHUB_COMMITS_JSON = json.dumps([
    {
        "html_url": "https://github.com/bitbug0x55AA/Blue_Team_Hunting_Field_Notes/commit/aaa111",
        "commit": {
            "message": "Add lateral movement via WMI\n\nDetailed notes on WMI-based lateral movement.",
            "author": {"date": "2024-04-01T08:00:00Z"},
        },
    },
    {
        "html_url": "https://github.com/bitbug0x55AA/Blue_Team_Hunting_Field_Notes/commit/bbb222",
        "commit": {
            "message": "Add persistence via registry run keys",
            "author": {"date": "2024-03-15T10:30:00Z"},
        },
    },
    {
        "html_url": "https://github.com/bitbug0x55AA/Blue_Team_Hunting_Field_Notes/commit/ccc333",
        "commit": {
            # Multi-line: only first line is the title
            "message": "Update README",
            "author": {"date": "2024-03-01T09:00:00Z"},
        },
    },
])

PENDING_MD = textwrap.dedent("""\
- [x] [Old Post](https://windowsir.blogspot.com/2023/12/userassist.html) — Windows IR
- [ ] [Another Post](https://windowsir.blogspot.com/2024/01/shimcache.html) — Windows IR
- [→] [Third Post](https://dfir.blog/prefetch-analysis/) — dfir.blog
""")


class TestParseBloggerFeed(unittest.TestCase):
    def test_returns_list_of_tuples(self):
        entries = ba.parse_blogger_feed(BLOGGER_FEED_XML)
        self.assertIsInstance(entries, list)

    def test_entry_count(self):
        entries = ba.parse_blogger_feed(BLOGGER_FEED_XML)
        self.assertEqual(len(entries), 2)

    def test_entry_shape(self):
        entries = ba.parse_blogger_feed(BLOGGER_FEED_XML)
        title, url, date = entries[0]
        self.assertEqual(title, "ShimCache and AppCompatCache")
        self.assertIn("windowsir.blogspot.com", url)
        self.assertEqual(date, "2024-01-15")

    def test_empty_feed_returns_empty_list(self):
        xml = '<?xml version="1.0"?><feed xmlns="http://www.w3.org/2005/Atom"></feed>'
        self.assertEqual(ba.parse_blogger_feed(xml), [])


class TestParseWordPressPosts(unittest.TestCase):
    def test_returns_list(self):
        entries = ba.parse_wordpress_posts(WORDPRESS_API_JSON)
        self.assertIsInstance(entries, list)

    def test_entry_count(self):
        entries = ba.parse_wordpress_posts(WORDPRESS_API_JSON)
        self.assertEqual(len(entries), 2)

    def test_entry_shape(self):
        entries = ba.parse_wordpress_posts(WORDPRESS_API_JSON)
        title, url, date = entries[0]
        self.assertEqual(title, "Prefetch Analysis")
        self.assertEqual(url, "https://dfir.blog/prefetch-analysis/")
        self.assertEqual(date, "2024-02-10")

    def test_empty_json_returns_empty_list(self):
        self.assertEqual(ba.parse_wordpress_posts("[]"), [])


class TestParseAtomFeed(unittest.TestCase):
    def test_returns_list(self):
        entries = ba.parse_atom_feed(GITHUB_ATOM_XML)
        self.assertIsInstance(entries, list)

    def test_entry_shape(self):
        entries = ba.parse_atom_feed(GITHUB_ATOM_XML)
        title, url, date = entries[0]
        self.assertEqual(title, "Create certutil.yml")
        self.assertIn("github.com", url)
        self.assertEqual(date, "2024-03-01")

    def test_updated_field_used_as_fallback(self):
        """Atom feeds use <updated> when <published> is absent."""
        entries = ba.parse_atom_feed(GITHUB_ATOM_XML)
        _, _, date = entries[0]
        self.assertRegex(date, r"\d{4}-\d{2}-\d{2}")


class TestParseGithubCommits(unittest.TestCase):
    def test_returns_list(self):
        entries = ba.parse_github_commits(GITHUB_COMMITS_JSON)
        self.assertIsInstance(entries, list)

    def test_entry_count(self):
        entries = ba.parse_github_commits(GITHUB_COMMITS_JSON)
        self.assertEqual(len(entries), 3)

    def test_entry_shape(self):
        entries = ba.parse_github_commits(GITHUB_COMMITS_JSON)
        title, url, date = entries[0]
        self.assertEqual(title, "Add lateral movement via WMI")
        self.assertIn("commit/aaa111", url)
        self.assertEqual(date, "2024-04-01")

    def test_multiline_message_uses_first_line_only(self):
        entries = ba.parse_github_commits(GITHUB_COMMITS_JSON)
        title, _, _ = entries[0]
        self.assertNotIn("\n", title)

    def test_empty_json_returns_empty(self):
        self.assertEqual(ba.parse_github_commits("[]"), [])

    def test_invalid_json_returns_empty(self):
        self.assertEqual(ba.parse_github_commits("not json"), [])


class TestLoadSeenUrls(unittest.TestCase):
    def setUp(self):
        import tempfile
        self.tmp = tempfile.mkdtemp()
        self.pending = os.path.join(self.tmp, "pending-review.md")
        with open(self.pending, "w") as f:
            f.write(PENDING_MD)

    def test_returns_set(self):
        seen = ba.load_seen_urls(self.pending)
        self.assertIsInstance(seen, set)

    def test_includes_all_marker_states(self):
        seen = ba.load_seen_urls(self.pending)
        # [x], [ ], and [→] entries all count as seen
        self.assertIn("https://windowsir.blogspot.com/2023/12/userassist.html", seen)
        self.assertIn("https://windowsir.blogspot.com/2024/01/shimcache.html", seen)
        self.assertIn("https://dfir.blog/prefetch-analysis/", seen)

    def test_missing_file_returns_empty_set(self):
        seen = ba.load_seen_urls(os.path.join(self.tmp, "nonexistent.md"))
        self.assertEqual(seen, set())

    def test_dedup_prevents_duplicate_entries(self):
        """Re-running fetch must not add URLs already in pending-review.md."""
        seen = ba.load_seen_urls(self.pending)
        entries = [
            ("ShimCache", "https://windowsir.blogspot.com/2024/01/shimcache.html", "2024-01-15"),
            ("New Post", "https://windowsir.blogspot.com/2024/06/new.html", "2024-06-01"),
        ]
        new = [e for e in entries if e[1] not in seen]
        self.assertEqual(len(new), 1)
        self.assertEqual(new[0][0], "New Post")


class TestClassifyBlogSource(unittest.TestCase):
    def test_blogger_recognized(self):
        self.assertEqual(ba.classify_blog_source("https://windowsir.blogspot.com/"), "blogger")

    def test_wordpress_recognized_by_path_hint(self):
        # WordPress sites expose /wp-json/ — classified by probing, but
        # known WordPress hosts get classified directly
        result = ba.classify_blog_source("https://thedfirreport.com/")
        self.assertIn(result, ("wordpress", "unknown"))

    def test_github_atom_recognized(self):
        result = ba.classify_blog_source("https://github.com/LOLBAS-Project/LOLBAS")
        self.assertEqual(result, "github")

    def test_unknown_for_generic_site(self):
        result = ba.classify_blog_source("https://example.com/")
        self.assertEqual(result, "unknown")


class TestRescanReviewedEntries(unittest.TestCase):
    """rescan_reviewed_entries() rewrites [x] → [ ] — no distinct [~] marker needed."""

    def _write_pending(self, tmp_path, content):
        with open(tmp_path, "w") as f:
            f.write(content)

    def test_reviewed_becomes_unreviewed(self):
        import tempfile
        with tempfile.NamedTemporaryFile(mode="w", suffix=".md", delete=False) as f:
            f.write("- [x] https://example.com/post1 <!-- reviewed -->\n")
            tmp = f.name
        count = ba.rescan_reviewed_entries(tmp)
        with open(tmp) as f:
            lines = f.readlines()
        self.assertEqual(count, 1)
        self.assertTrue(lines[0].startswith("- [ ] "), f"expected [ ], got: {lines[0]!r}")

    def test_task_created_entries_unchanged(self):
        import tempfile
        with tempfile.NamedTemporaryFile(mode="w", suffix=".md", delete=False) as f:
            f.write("- [→] https://example.com/post2\n")
            tmp = f.name
        count = ba.rescan_reviewed_entries(tmp)
        with open(tmp) as f:
            content = f.read()
        self.assertEqual(count, 0)
        self.assertIn("[→]", content)

    def test_missing_file_returns_zero(self):
        count = ba.rescan_reviewed_entries("/nonexistent/path/pending.md")
        self.assertEqual(count, 0)


class TestPendingFileLock(unittest.TestCase):
    """locked_write(path, transform_fn) serializes concurrent read-modify-writes."""

    def setUp(self):
        import tempfile
        self.tmp = tempfile.mkdtemp()
        self.path = os.path.join(self.tmp, "pending-review.md")
        with open(self.path, "w") as f:
            f.write("- [ ] https://example.com/post1\n")

    def test_locked_write_applies_transform(self):
        """transform_fn receives current content, returns new content."""
        ba.locked_write(self.path, lambda c: c + "- [ ] https://example.com/post2\n")
        with open(self.path) as f:
            content = f.read()
        self.assertIn("post1", content)
        self.assertIn("post2", content)

    def test_locked_write_is_atomic(self):
        """Two concurrent locked_write calls both apply without losing data."""
        import threading
        results = []

        def append(url):
            ba.locked_write(self.path, lambda c: c + f"- [ ] {url}\n")
            results.append(url)

        t1 = threading.Thread(target=append, args=("https://a.com/1",))
        t2 = threading.Thread(target=append, args=("https://b.com/2",))
        t1.start(); t2.start()
        t1.join(); t2.join()

        with open(self.path) as f:
            content = f.read()
        self.assertIn("https://a.com/1", content)
        self.assertIn("https://b.com/2", content)

    def test_lock_file_cleaned_up_after_write(self):
        """Lock file must not linger after locked_write completes."""
        lock_path = self.path + ".lock"
        ba.locked_write(self.path, lambda c: c)
        self.assertFalse(os.path.exists(lock_path),
                         "stale lockfile left behind after write")

    def test_stale_lock_from_dead_pid_is_stolen(self):
        """A lockfile containing a dead PID must not block forever."""
        lock_path = self.path + ".lock"
        # Write a lockfile with PID 1 (init/launchd — never our process)
        # On all platforms PID 1 exists but is not our process, so it's
        # effectively "dead" from our perspective for stealing purposes.
        # Use a guaranteed-dead PID instead: 99999999
        with open(lock_path, "w") as f:
            f.write("99999999")
        # Should complete without hanging
        ba.locked_write(self.path, lambda c: c + "- [ ] https://example.com/stolen\n")
        with open(self.path) as f:
            content = f.read()
        self.assertIn("stolen", content)
        self.assertFalse(os.path.exists(lock_path))



class TestParseRssXml(unittest.TestCase):
    """parse_rss_xml(text) — unified RSS/Atom parser used by xml-first strategy."""

    def test_parses_blogger_atom_feed(self):
        entries = ba.parse_rss_xml(BLOGGER_FEED_XML)
        self.assertEqual(len(entries), 2)
        title, url, date = entries[0]
        self.assertEqual(title, "ShimCache and AppCompatCache")
        self.assertIn("windowsir.blogspot.com", url)
        self.assertEqual(date, "2024-01-15")

    def test_parses_generic_atom_feed(self):
        entries = ba.parse_rss_xml(GITHUB_ATOM_XML)
        self.assertEqual(len(entries), 1)
        title, url, date = entries[0]
        self.assertEqual(title, "Create certutil.yml")
        self.assertIn("github.com", url)

    def test_empty_text_returns_empty_list(self):
        self.assertEqual(ba.parse_rss_xml(""), [])

    def test_invalid_xml_returns_empty_list(self):
        self.assertEqual(ba.parse_rss_xml("not xml at all"), [])

    def test_ghost_rss_feed(self):
        """Ghost blogs emit RSS 2.0 (not Atom) — parse_rss_xml must handle it."""
        ghost_rss = textwrap.dedent("""\
        <?xml version="1.0" encoding="UTF-8"?>
        <rss version="2.0">
          <channel>
            <title>dfir.blog</title>
            <item>
              <title>SRUM Database Forensics</title>
              <link>https://dfir.blog/srum-database/</link>
              <pubDate>Fri, 10 Feb 2024 09:00:00 +0000</pubDate>
            </item>
            <item>
              <title>Prefetch Analysis</title>
              <link>https://dfir.blog/prefetch-analysis/</link>
              <pubDate>Sun, 20 Jan 2024 09:00:00 +0000</pubDate>
            </item>
          </channel>
        </rss>
        """)
        entries = ba.parse_rss_xml(ghost_rss)
        self.assertEqual(len(entries), 2)
        title, url, date = entries[0]
        self.assertEqual(title, "SRUM Database Forensics")
        self.assertEqual(url, "https://dfir.blog/srum-database/")
        self.assertEqual(date, "2024-02-10")

    def test_rss_pubdate_parsed_to_ymd(self):
        """RSS 2.0 <pubDate> uses RFC 2822 format — must be normalised to YYYY-MM-DD."""
        rss = textwrap.dedent("""\
        <?xml version="1.0" encoding="UTF-8"?>
        <rss version="2.0"><channel>
          <item>
            <title>Post</title>
            <link>https://example.com/post</link>
            <pubDate>Mon, 01 Apr 2024 12:00:00 GMT</pubDate>
          </item>
        </channel></rss>
        """)
        entries = ba.parse_rss_xml(rss)
        self.assertEqual(len(entries), 1)
        _, _, date = entries[0]
        self.assertRegex(date, r"\d{4}-\d{2}-\d{2}")


class TestXmlUrlFirstStrategy(unittest.TestCase):
    """
    fetch_all_sources should treat xmlUrl as authoritative for non-Blogger,
    non-GitHub, non-YouTube sources.  WordPress REST API is a fallback only
    when xmlUrl returns nothing at all — not when it returns a full page.
    """

    def test_skip_titles_includes_abuse_ch_blog(self):
        """abuse.ch blog is an IOC-adjacent feed — not artifact documentation."""
        self.assertIn("abuse.ch blog", ba._SKIP_TITLES)

    def test_xmlurl_strategy_constant_exists(self):
        """Module must expose XMLURL_ONLY_PLATFORMS documenting non-WP platforms."""
        # Ghost, HubSpot, Squarespace, Jekyll, Hugo have no WP REST API —
        # trying it generates noisy 404s.  The strategy document is these platforms.
        self.assertTrue(
            hasattr(ba, "_XMLURL_ONLY_PLATFORMS"),
            "_XMLURL_ONLY_PLATFORMS constant must exist in fetch_all_sources",
        )

    def test_xmlurl_only_platforms_covers_ghost(self):
        """Ghost is not WordPress — WP fallback must be skipped."""
        self.assertIn("ghost.io", ba._XMLURL_ONLY_PLATFORMS)

    def test_xmlurl_only_platforms_covers_squarespace(self):
        self.assertIn("squarespace.com", ba._XMLURL_ONLY_PLATFORMS)

    def test_xmlurl_only_platforms_covers_hubspot(self):
        self.assertIn("hubspot.com", ba._XMLURL_ONLY_PLATFORMS)

    def test_classify_ghost_blog_returns_unknown_not_wordpress(self):
        """dfir.blog is Ghost — classify_blog_source must NOT return 'wordpress'."""
        # Ghost blogs look like any other HTTPS site; classify must not
        # assume WordPress for unknown CMS.
        result = ba.classify_blog_source("https://dfir.blog/")
        self.assertNotEqual(result, "wordpress")

    def test_should_try_wordpress_false_when_xmlurl_has_entries(self):
        """Helper: if xmlUrl returned entries, WP fallback must not be tried."""
        # This is the core logic fix: entries present → skip WP API
        self.assertFalse(ba._should_try_wordpress(entries=["anything"], xml_url="https://x.com/feed/"))

    def test_should_try_wordpress_true_when_no_entries_and_xml_url(self):
        """Helper: if xmlUrl returned nothing, try WP as fallback."""
        self.assertTrue(ba._should_try_wordpress(entries=[], xml_url="https://x.com/feed/"))

    def test_should_try_wordpress_false_when_no_xml_url(self):
        """No xmlUrl and no entries — can't help, don't try random WP endpoints."""
        self.assertFalse(ba._should_try_wordpress(entries=[], xml_url=""))


class TestDetectIsWordpress(unittest.TestCase):
    """detect_is_wordpress(xml_text) — True iff RSS feed contains WP generator tag."""

    def test_wordpress_generator_tag_detected(self):
        rss = textwrap.dedent("""\
        <?xml version="1.0"?>
        <rss version="2.0">
          <channel>
            <generator>https://wordpress.org/?v=6.5.3</generator>
            <item><title>T</title><link>https://example.com/</link></item>
          </channel>
        </rss>
        """)
        self.assertTrue(ba.detect_is_wordpress(rss))

    def test_wordpress_generator_http_also_detected(self):
        rss = textwrap.dedent("""\
        <?xml version="1.0"?>
        <rss version="2.0">
          <channel>
            <generator>http://wordpress.org/?v=5.9</generator>
          </channel>
        </rss>
        """)
        self.assertTrue(ba.detect_is_wordpress(rss))

    def test_ghost_generator_not_wordpress(self):
        rss = textwrap.dedent("""\
        <?xml version="1.0"?>
        <rss version="2.0">
          <channel>
            <generator>Ghost 5.79</generator>
          </channel>
        </rss>
        """)
        self.assertFalse(ba.detect_is_wordpress(rss))

    def test_no_generator_tag_returns_false(self):
        rss = textwrap.dedent("""\
        <?xml version="1.0"?>
        <rss version="2.0"><channel><item><title>T</title></item></channel></rss>
        """)
        self.assertFalse(ba.detect_is_wordpress(rss))

    def test_empty_string_returns_false(self):
        self.assertFalse(ba.detect_is_wordpress(""))

    def test_jekyll_feed_not_wordpress(self):
        """Jekyll Atom feed — must not be misdetected as WordPress."""
        self.assertFalse(ba.detect_is_wordpress(BLOGGER_FEED_XML))


class TestReadOpmlUsesTextAttribute(unittest.TestCase):
    """read_opml must use `text` (canonical OPML identifier) over `title`.

    The OPML spec defines `text` as the required display attribute.
    `title` is optional and may contain a verbose description.
    _SKIP_TITLES matches against short names stored in `text`.
    """

    _OPML_MIXED_ATTRS = textwrap.dedent("""\
    <?xml version="1.0" encoding="UTF-8"?>
    <opml version="2.0">
      <head><title>Test</title></head>
      <body>
        <!-- text == title -->
        <outline type="rss" text="Windows Incident Response"
                 title="Windows Incident Response"
                 xmlUrl="https://windowsir.blogspot.com/feeds/posts/default"
                 htmlUrl="https://windowsir.blogspot.com/"/>
        <!-- text != title: IOC feed with verbose title -->
        <outline type="rss" text="URLhaus"
                 title="URLhaus — malware distribution URLs"
                 xmlUrl="https://urlhaus.abuse.ch/rss/"
                 htmlUrl="https://urlhaus.abuse.ch/"/>
        <!-- text only (no title attr) -->
        <outline type="rss" text="MalwareBazaar"
                 xmlUrl="https://bazaar.abuse.ch/rss/"
                 htmlUrl="https://bazaar.abuse.ch/"/>
      </body>
    </opml>
    """)

    def test_text_attr_is_used_for_source_name(self):
        """When text != title, read_opml must return the `text` value as title."""
        import tempfile, os
        with tempfile.NamedTemporaryFile(mode="w", suffix=".opml", delete=False) as f:
            f.write(self._OPML_MIXED_ATTRS)
            path = f.name
        try:
            sources = ba.read_opml(path)
            titles = [s["title"] for s in sources]
            # text="URLhaus" must be used, not title="URLhaus — malware distribution URLs"
            self.assertIn("URLhaus", titles,
                          "read_opml must use `text` attribute, not the verbose `title`")
            self.assertNotIn("URLhaus — malware distribution URLs", titles,
                             "verbose title must not appear — use `text` for the source name")
        finally:
            os.unlink(path)

    def test_text_attr_enables_skip_list_matching(self):
        """_SKIP_TITLES must match when OPML entry has verbose title but short text."""
        import tempfile, os
        with tempfile.NamedTemporaryFile(mode="w", suffix=".opml", delete=False) as f:
            f.write(self._OPML_MIXED_ATTRS)
            path = f.name
        try:
            sources = ba.read_opml(path)
            non_skipped = [s for s in sources if s["title"] not in ba._SKIP_TITLES]
            skipped_count = len(sources) - len(non_skipped)
            # URLhaus + MalwareBazaar are in _SKIP_TITLES; should be skipped
            self.assertEqual(skipped_count, 2,
                             f"2 IOC sources should be skipped; non-skipped: {[s['title'] for s in non_skipped]}")
        finally:
            os.unlink(path)

    def test_fallback_to_title_when_no_text(self):
        """If `text` is absent, fall back to `title` attribute."""
        import tempfile, os
        opml = textwrap.dedent("""\
        <?xml version="1.0"?>
        <opml version="2.0">
          <body>
            <outline type="rss" title="OnlyTitle"
                     xmlUrl="https://example.com/feed/" htmlUrl="https://example.com/"/>
          </body>
        </opml>
        """)
        with tempfile.NamedTemporaryFile(mode="w", suffix=".opml", delete=False) as f:
            f.write(opml)
            path = f.name
        try:
            sources = ba.read_opml(path)
            self.assertEqual(sources[0]["title"], "OnlyTitle")
        finally:
            os.unlink(path)


_SANS_HTML_FIXTURE = textwrap.dedent("""\
<!DOCTYPE html><html><head><title>SANS Blog</title></head><body>
<script>window.__remixContext = {"state":{"loaderData":{"routes/blog/index":
{"results":[{"hits":[
{"contentType":"blog_single_page","title":"Living off the Cloud",
 "url":"/blog/living-off-the-cloud","createdAt":"2026-04-20T23:47:12.425Z",
 "description":"Cloud services abused for C2 and data exfiltration."},
{"contentType":"blog_single_page","title":"ShimCache Forensics Deep Dive",
 "url":"/blog/shimcache-forensics-deep-dive","createdAt":"2026-03-15T10:00:00.000Z",
 "description":"How ShimCache tracks execution."},
{"contentType":"blog_single_page","title":"Windows Prefetch Analysis",
 "url":"/blog/windows-prefetch-analysis","createdAt":"2026-02-01T08:00:00.000Z",
 "description":"Prefetch artifact analysis."}
],"nbHits":949}]}}}}</script>
<main>
<a href="/blog/living-off-the-cloud">Living off the Cloud</a>
<a href="/blog/shimcache-forensics-deep-dive">ShimCache Forensics Deep Dive</a>
</main>
</body></html>
""")


class TestParseSansBlogHtml(unittest.TestCase):
    """parse_sans_blog_html(html) → list[(title, url, date)]

    SANS blog embeds a JSON blob in a <script> tag (Remix/ContentStack SSR).
    Pure-logic parser — no HTTP.
    """

    def test_returns_list_of_tuples(self):
        result = ba.parse_sans_blog_html(_SANS_HTML_FIXTURE)
        self.assertIsInstance(result, list)
        for item in result:
            self.assertEqual(len(item), 3)

    def test_parses_three_posts(self):
        result = ba.parse_sans_blog_html(_SANS_HTML_FIXTURE)
        self.assertEqual(len(result), 3)

    def test_url_is_absolute(self):
        result = ba.parse_sans_blog_html(_SANS_HTML_FIXTURE)
        for _, url, _ in result:
            self.assertTrue(url.startswith("https://www.sans.org/blog/"),
                            f"URL must be absolute: {url}")

    def test_date_is_yyyy_mm_dd(self):
        result = ba.parse_sans_blog_html(_SANS_HTML_FIXTURE)
        import re
        for _, _, date in result:
            self.assertRegex(date, r"^\d{4}-\d{2}-\d{2}$",
                             f"Date must be YYYY-MM-DD: {date}")

    def test_first_post_title_correct(self):
        result = ba.parse_sans_blog_html(_SANS_HTML_FIXTURE)
        self.assertEqual(result[0][0], "Living off the Cloud")

    def test_empty_html_returns_empty(self):
        self.assertEqual(ba.parse_sans_blog_html(""), [])

    def test_no_hits_returns_empty(self):
        self.assertEqual(ba.parse_sans_blog_html("<html><body>no data</body></html>"), [])

    def test_skips_non_blog_content_types(self):
        """Only contentType=blog_single_page entries should be included."""
        html = textwrap.dedent("""\
        <script>{"results":[{"hits":[
          {"contentType":"page","title":"About","url":"/about","createdAt":"2026-01-01T00:00:00Z"},
          {"contentType":"blog_single_page","title":"DFIR Post","url":"/blog/dfir-post","createdAt":"2026-01-02T00:00:00Z"}
        ]}]}</script>
        """)
        result = ba.parse_sans_blog_html(html)
        titles = [t for t, _, _ in result]
        self.assertNotIn("About", titles)
        self.assertIn("DFIR Post", titles)


if __name__ == "__main__":
    unittest.main()
