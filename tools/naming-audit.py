#!/usr/bin/env python3
"""Read-only inventory of documented legacy naming compatibility references.

This is a source audit, not a behavioral test or a release gate. Exceptions
are reviewed in docs/msc/naming-migration.md; there is no automatic accept mode.
"""

from __future__ import annotations

from collections import Counter
import json
from pathlib import Path
import re
import subprocess
import sys


ROOT = Path(__file__).resolve().parents[1]
INVENTORY = "docs/msc/naming-compatibility.json"
DOCUMENTED_CHANGES = {
    INVENTORY,
    "docs/msc/naming-migration.md",
    "tools/naming-audit.py",
    "AGENTS.md",
}
HISTORICAL_PREFIXES = (
    "docs/msc/audit/",
    "docs/msc/release/",
    "docs/msc/archive/",
    "docs/msc/archives/",
    ".github/archive/",
    "tools/release/archive/",
    "corpus/",
    "third_party/",
)
HISTORICAL_FILES = {"docs/msc/rolling-plan-archive.md"}
LEGACY = re.compile(r"msc ?2|agent", re.IGNORECASE)
TOKEN = re.compile(r"[A-Za-z0-9_.:-]+(?: 2\b)?")
# These words contain the same letters but are unrelated to service naming.
ORDINARY_WORDS = {"magenta", "dragenter", "ondragenter"}


def historical(name: str) -> bool:
    return name in HISTORICAL_FILES or name.startswith(HISTORICAL_PREFIXES)


def source_files() -> list[str]:
    result = subprocess.run(
        ["git", "ls-files", "-z", "--cached", "--others", "--exclude-standard"],
        cwd=ROOT, check=True, stdout=subprocess.PIPE,
    )
    return sorted(set(result.stdout.decode("utf-8").rstrip("\0").split("\0")))


def references() -> dict[str, dict[str, int]]:
    found = {}
    for name in source_files():
        if name in DOCUMENTED_CHANGES or historical(name):
            continue
        path = ROOT / name
        if not path.is_file():
            continue
        try:
            contents = path.read_text(encoding="utf-8-sig")
        except UnicodeError:
            continue
        counts = Counter(
            token for token in TOKEN.findall(contents)
            if LEGACY.search(token) and token.lower().split("::")[-1] not in ORDINARY_WORDS
        )
        # Paths are audited as well as their contents.
        counts.update(
            "path:" + token for token in TOKEN.findall(name)
            if LEGACY.search(token)
        )
        if counts:
            found[name] = dict(sorted(counts.items()))
    return found


def main() -> int:
    inventory = json.loads((ROOT / INVENTORY).read_text(encoding="utf-8"))
    expected = {
        name: {entry["token"]: entry["count"] for entry in entries}
        for name, entries in inventory["files"].items()
    }
    actual = references()
    problems = []
    for name in sorted(set(actual) | set(expected)):
        before, after = expected.get(name, {}), actual.get(name, {})
        for token in sorted(set(before) | set(after)):
            if before.get(token, 0) != after.get(token, 0):
                problems.append(
                    f"{name}: {token!r}: documented {before.get(token, 0)}, "
                    f"found {after.get(token, 0)}"
                )
    if problems:
        print("Naming inventory differs; inspect the change and its compatibility reason.")
        print("\n".join(problems))
        return 1
    print(
        f"Naming audit passed: {len(actual)} files contain only inventoried "
        "compatibility references; historical evidence is explicitly excluded."
    )
    return 0


if __name__ == "__main__":
    sys.exit(main())
