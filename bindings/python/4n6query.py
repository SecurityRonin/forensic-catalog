#!/usr/bin/env python3
"""4n6query — Python interface to the forensicnomicon DFIR catalog.

This script is a thin wrapper around the compiled ``4n6query`` Rust binary.
It passes all arguments through unchanged and forwards stdout/stderr/exit code
transparently, so every subcommand, flag, and output format works identically
to the Rust binary.

Binary resolution order:
1. ``FORENSICNOMICON_BIN`` environment variable (set by tests / CI)
2. ``4n6query`` on ``PATH`` (e.g. after ``cargo install --path crates/4n6query``)
3. ``target/debug/4n6query`` relative to the repo root (dev builds)
4. ``target/release/4n6query`` relative to the repo root (release builds)

Usage (identical to the Rust binary)::

    python 4n6query.py lolbas lookup windows certutil.exe
    python 4n6query.py sites lookup raw.githubusercontent.com --format json
    python 4n6query.py catalog search prefetch
    python 4n6query.py catalog show userassist_exe --format json
    python 4n6query.py catalog mitre T1547.001
    python 4n6query.py catalog triage --format json
    python 4n6query.py catalog list
    python 4n6query.py dump --format json --dataset all
"""

from __future__ import annotations

import os
import shutil
import subprocess
import sys
from pathlib import Path

# ---------------------------------------------------------------------------
# Binary resolution
# ---------------------------------------------------------------------------

_BINARY_NAME = "4n6query"


def _repo_root() -> Path:
    """Return the repo root (three levels up from this file's directory)."""
    return Path(__file__).resolve().parents[2]


def _find_binary() -> str:
    """Return the path to the 4n6query binary.

    Raises SystemExit with a helpful message if the binary cannot be found.
    """
    # 1. Explicit override (used by tests and CI)
    env_bin = os.environ.get("FORENSICNOMICON_BIN")
    if env_bin and Path(env_bin).is_file():
        return env_bin

    # 2. System PATH
    path_bin = shutil.which(_BINARY_NAME)
    if path_bin:
        return path_bin

    # 3. Debug build relative to repo root
    repo = _repo_root()
    debug_bin = repo / "target" / "debug" / _BINARY_NAME
    if debug_bin.is_file():
        return str(debug_bin)

    # 4. Release build
    release_bin = repo / "target" / "release" / _BINARY_NAME
    if release_bin.is_file():
        return str(release_bin)

    print(
        f"error: '{_BINARY_NAME}' binary not found.\n"
        "Build it with: cargo build -p forensicnomicon-cli\n"
        "or install it: cargo install --path crates/4n6query",
        file=sys.stderr,
    )
    sys.exit(1)


# ---------------------------------------------------------------------------
# Entry point
# ---------------------------------------------------------------------------


def main() -> None:
    binary = _find_binary()
    result = subprocess.run([binary, *sys.argv[1:]])
    sys.exit(result.returncode)


if __name__ == "__main__":
    main()
