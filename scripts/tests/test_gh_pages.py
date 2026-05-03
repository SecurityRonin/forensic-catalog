"""
RED tests for the GH Pages search widget.

Verifies:
  1. docs/search.html exists with required structural elements
  2. docs.yml workflow generates data.json before uploading the Pages artifact
  3. The JSON data structure matches what the widget expects
"""

import json
import os
import re
import subprocess
import sys
import unittest

REPO = os.path.abspath(os.path.join(os.path.dirname(__file__), "..", ".."))
SEARCH_HTML = os.path.join(REPO, "docs", "search.html")
DOCS_YML = os.path.join(REPO, ".github", "workflows", "docs.yml")


class TestSearchHtmlExists(unittest.TestCase):
    def test_search_html_exists(self):
        self.assertTrue(os.path.exists(SEARCH_HTML), "docs/search.html must exist")

    def test_has_doctype(self):
        with open(SEARCH_HTML) as f:
            content = f.read()
        self.assertIn("<!DOCTYPE html>", content)

    def test_has_search_input(self):
        with open(SEARCH_HTML) as f:
            content = f.read()
        self.assertRegex(content, r'<input[^>]+id=["\']?search', "must have a search input")

    def test_has_platform_filter(self):
        with open(SEARCH_HTML) as f:
            content = f.read()
        self.assertIn("platform", content.lower(), "must have a platform filter")

    def test_has_triage_filter(self):
        with open(SEARCH_HTML) as f:
            content = f.read()
        self.assertIn("triage", content.lower(), "must have a triage filter")

    def test_has_mitre_filter(self):
        with open(SEARCH_HTML) as f:
            content = f.read()
        self.assertIn("mitre", content.lower(), "must have a MITRE filter")

    def test_loads_data_json(self):
        with open(SEARCH_HTML) as f:
            content = f.read()
        self.assertIn("data.json", content, "must fetch data.json")

    def test_no_external_js_frameworks(self):
        """Widget must be pure vanilla JS — no React/Vue/jQuery."""
        with open(SEARCH_HTML) as f:
            content = f.read()
        for framework in ("react", "vue", "jquery", "angular", "svelte"):
            self.assertNotIn(framework, content.lower(),
                             f"must not load {framework}")

    def test_results_container_present(self):
        with open(SEARCH_HTML) as f:
            content = f.read()
        self.assertRegex(content, r'id=["\']?results', "must have a #results container")


class TestDocsWorkflowGeneratesJson(unittest.TestCase):
    def _workflow_content(self):
        with open(DOCS_YML) as f:
            return f.read()

    def test_workflow_runs_dump_command(self):
        content = self._workflow_content()
        self.assertIn("dump", content,
                      "docs.yml must run 4n6query dump to generate data.json")

    def test_workflow_outputs_data_json(self):
        content = self._workflow_content()
        self.assertIn("data.json", content,
                      "docs.yml must reference data.json in its steps")

    def test_data_json_copied_to_doc_dir(self):
        """data.json must land inside target/doc/ so it's included in Pages artifact."""
        content = self._workflow_content()
        self.assertIn("target/doc", content)
        # Either cp data.json or write directly into target/doc
        self.assertTrue(
            "target/doc/data.json" in content or
            ("data.json" in content and "target/doc" in content),
            "data.json must be written into target/doc/",
        )


class TestDataJsonStructure(unittest.TestCase):
    """
    Run `cargo run -p forensicnomicon-cli -- dump --format json --dataset all`
    and verify the JSON structure the search widget expects.
    """

    @classmethod
    def setUpClass(cls):
        result = subprocess.run(
            ["cargo", "run", "-p", "forensicnomicon-cli", "--",
             "dump", "--format", "json", "--dataset", "all"],
            capture_output=True, text=True, cwd=REPO,
        )
        cls.exit_code = result.returncode
        try:
            cls.data = json.loads(result.stdout)
        except json.JSONDecodeError:
            cls.data = None

    def test_dump_exits_zero(self):
        self.assertEqual(self.exit_code, 0)

    def test_data_is_dict(self):
        self.assertIsInstance(self.data, dict)

    def test_has_lolbas_key(self):
        self.assertIn("lolbas_windows", self.data)

    def test_has_abusable_sites_key(self):
        self.assertIn("abusable_sites", self.data)

    def test_lolbas_entries_have_name(self):
        entries = self.data.get("lolbas_windows", [])
        self.assertTrue(len(entries) > 0)
        self.assertIn("name", entries[0])

    def test_lolbas_entries_have_mitre(self):
        entries = self.data.get("lolbas_windows", [])
        self.assertIn("mitre_techniques", entries[0])


if __name__ == "__main__":
    unittest.main()
