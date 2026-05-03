"""
RED tests for the forensicnomicon PyO3 native Python extension.

All 12 tests must fail with ModuleNotFoundError before implementation.
"""

import pytest

# Test 1: import succeeds
def test_import():
    import forensicnomicon  # noqa: F401

# Test 2: __version__ is a string
def test_version_is_string():
    import forensicnomicon
    assert isinstance(forensicnomicon.__version__, str)
    assert len(forensicnomicon.__version__) > 0

# Test 3: lolbas_lookup returns non-empty list for known binary
def test_lolbas_lookup_known_binary():
    import forensicnomicon
    entries = forensicnomicon.lolbas_lookup("certutil.exe")
    assert isinstance(entries, list)
    assert len(entries) > 0

# Test 4: each entry has name, platform, mitre_techniques keys
def test_lolbas_lookup_entry_keys():
    import forensicnomicon
    entries = forensicnomicon.lolbas_lookup("certutil.exe")
    for entry in entries:
        assert "name" in entry
        assert "platform" in entry
        assert "mitre_techniques" in entry

# Test 5: lolbas_lookup with platform="windows" returns only windows entries
def test_lolbas_lookup_platform_filter():
    import forensicnomicon
    entries = forensicnomicon.lolbas_lookup("certutil.exe", platform="windows")
    assert len(entries) > 0
    for entry in entries:
        assert entry["platform"] == "windows"

# Test 6: lolbas_lookup for nonexistent binary returns empty list
def test_lolbas_lookup_unknown_binary():
    import forensicnomicon
    entries = forensicnomicon.lolbas_lookup("nonexistent_binary_xyz")
    assert isinstance(entries, list)
    assert len(entries) == 0

# Test 7: catalog_search returns list with >=1 result for "userassist"
def test_catalog_search_userassist():
    import forensicnomicon
    results = forensicnomicon.catalog_search("userassist")
    assert isinstance(results, list)
    assert len(results) >= 1

# Test 8: each catalog entry has id, name, triage_priority, mitre_techniques
def test_catalog_search_entry_keys():
    import forensicnomicon
    results = forensicnomicon.catalog_search("userassist")
    for entry in results:
        assert "id" in entry
        assert "name" in entry
        assert "triage_priority" in entry
        assert "mitre_techniques" in entry

# Test 9: catalog_show returns dict for known artifact
def test_catalog_show_known():
    import forensicnomicon
    artifact = forensicnomicon.catalog_show("userassist_exe")
    assert artifact is not None
    assert isinstance(artifact, dict)

# Test 10: catalog_show returns None for unknown artifact
def test_catalog_show_unknown():
    import forensicnomicon
    artifact = forensicnomicon.catalog_show("this_does_not_exist_xyz")
    assert artifact is None

# Test 11: triage_list returns list, first entry is Critical priority
def test_triage_list():
    import forensicnomicon
    triage = forensicnomicon.triage_list()
    assert isinstance(triage, list)
    assert len(triage) > 0
    assert triage[0]["triage_priority"] == "Critical"

# Test 12: sites_lookup for known domain returns non-empty list
def test_sites_lookup_known():
    import forensicnomicon
    sites = forensicnomicon.sites_lookup("raw.githubusercontent.com")
    assert isinstance(sites, list)
    assert len(sites) > 0
