"""
Tests for scripts/check_feed_updates.py

Pure-logic tests — no real HTTP calls.
"""

import os
import sys
import unittest

sys.path.insert(0, os.path.join(os.path.dirname(__file__), ".."))

import check_feed_updates as cfu  # noqa: E402


class TestNoReviewList(unittest.TestCase):
    """_NO_PENDING_REVIEW must exist and cover IOC/LOL-dataset sources."""

    def test_no_pending_review_constant_exists(self):
        self.assertTrue(
            hasattr(cfu, "_NO_PENDING_REVIEW"),
            "_NO_PENDING_REVIEW frozenset must exist in check_feed_updates",
        )

    def test_no_pending_review_is_frozenset(self):
        self.assertIsInstance(cfu._NO_PENDING_REVIEW, frozenset)

    def test_ioc_feeds_excluded(self):
        """URLhaus, MalwareBazaar, ThreatFox produce IOC entries, not artifact docs."""
        for title in ("URLhaus", "MalwareBazaar", "ThreatFox"):
            self.assertIn(
                title,
                cfu._NO_PENDING_REVIEW,
                f"'{title}' should be in _NO_PENDING_REVIEW — it is an IOC feed, not a DFIR blog",
            )

    def test_lol_dataset_commits_excluded(self):
        """LOL dataset GitHub commit feeds generate one entry per binary added.
        These are handled by fetch_*.py scripts and should not flood pending-review.md."""
        for title in ("LOLBAS Project (Windows)", "GTFOBins (Linux)", "LOOBins (macOS)", "LOLDrivers (BYOVD)", "LOFL Project (RMM C2 indicators)"):
            self.assertIn(
                title,
                cfu._NO_PENDING_REVIEW,
                f"'{title}' should be in _NO_PENDING_REVIEW — LOL dataset commits, not blog posts",
            )

    def test_misp_taxonomies_excluded(self):
        """MISP taxonomy commits are CI/tooling changes, not DFIR artifact documentation."""
        self.assertIn(
            "MISP Taxonomies",
            cfu._NO_PENDING_REVIEW,
            "'MISP Taxonomies' should be in _NO_PENDING_REVIEW",
        )

    def test_dfir_blog_feeds_not_excluded(self):
        """Real DFIR blogs must NOT be in _NO_PENDING_REVIEW."""
        for title in ("Windows Incident Response", "The DFIR Report", "Andrea Fortuna", "13cubed"):
            self.assertNotIn(
                title,
                cfu._NO_PENDING_REVIEW,
                f"'{title}' is a real DFIR blog and must not be in _NO_PENDING_REVIEW",
            )


class TestFilterBeforePending(unittest.TestCase):
    """new_entries must be filtered through _NO_PENDING_REVIEW before append_pending_review."""

    def _make_entries(self, *source_titles):
        """Return [(source, title, url)] tuples for given source names."""
        return [(src, f"Post from {src}", f"https://example.com/{src}") for src in source_titles]

    def test_filter_removes_no_review_entries(self):
        """filter_pending_entries() must drop entries whose source is in _NO_PENDING_REVIEW."""
        entries = self._make_entries("URLhaus", "Windows Incident Response", "MalwareBazaar")
        result = cfu.filter_pending_entries(entries)
        sources = [e[0] for e in result]
        self.assertNotIn("URLhaus", sources)
        self.assertNotIn("MalwareBazaar", sources)
        self.assertIn("Windows Incident Response", sources)

    def test_filter_keeps_lol_dataset_entries_out(self):
        entries = self._make_entries("LOLBAS Project (Windows)", "GTFOBins (Linux)", "Didier Stevens Blog")
        result = cfu.filter_pending_entries(entries)
        sources = [e[0] for e in result]
        self.assertNotIn("LOLBAS Project (Windows)", sources)
        self.assertNotIn("GTFOBins (Linux)", sources)
        self.assertIn("Didier Stevens Blog", sources)

    def test_filter_empty_input(self):
        self.assertEqual(cfu.filter_pending_entries([]), [])

    def test_filter_all_excluded(self):
        entries = self._make_entries("URLhaus", "MalwareBazaar", "ThreatFox")
        self.assertEqual(cfu.filter_pending_entries(entries), [])

    def test_filter_none_excluded(self):
        entries = self._make_entries("Windows Incident Response", "The DFIR Report")
        result = cfu.filter_pending_entries(entries)
        self.assertEqual(len(result), 2)


class TestBlueteamFieldNotesIncluded(unittest.TestCase):
    """Blue Team Field Notes is a legitimate DFIR notebook — must NOT be excluded."""

    def test_blue_team_field_notes_not_excluded(self):
        self.assertNotIn(
            "Blue_Team_Hunting_Field_Notes",
            cfu._NO_PENDING_REVIEW,
        )
        # Also check the title variant used in the OPML
        self.assertNotIn(
            "Blue Team Hunting Field Notes (bitbug0x55AA)",
            cfu._NO_PENDING_REVIEW,
        )
