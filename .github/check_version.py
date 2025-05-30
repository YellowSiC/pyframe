#!/usr/bin/env python3
"""
Check that the version in Cargo.toml matches the version from the GITHUB_REF environment variable.
"""

import logging
import os
import re
import sys
from pathlib import Path


def main() -> int:
    # Logging für bessere Struktur
    logging.basicConfig(format="%(levelname)s: %(message)s", level=logging.INFO)

    cargo_path = Path("Cargo.toml")
    if not cargo_path.is_file():
        logging.error('Path "%s" does not exist.', cargo_path)
        return 1

    # Lese Version von GitHub-Tag (z.B. refs/tags/v1.2.3 oder v1.2.3)
    version_ref = os.getenv("GITHUB_REF")
    if version_ref:
        # Entfernt "refs/tags/" und führende "v" (optional)
        version = re.sub(r"^refs/tags/v*", "", version_ref.lower())
    else:
        logging.error('"GITHUB_REF" environment variable not found.')
        return 1

    # Umwandlung von Python pre-release in Rust Syntax
    version = version.replace("a", "-alpha").replace("b", "-beta")

    # Suche nach Version in Cargo.toml
    cargo_content = cargo_path.read_text()
    version_regex = re.compile(r"""^version\s*=\s*["'](.+?)["']""", re.M)

    match = version_regex.search(cargo_content)
    if not match:
        logging.error('Could not find version in "%s".', cargo_path)
        return 1

    cargo_version = match.group(1)
    if cargo_version == version:
        logging.info(
            '✓ Version in GITHUB_REF matches Cargo.toml version: "%s"', cargo_version
        )
        return 0
    else:
        logging.error(
            'Version mismatch: GITHUB_REF version "%s" does not match Cargo.toml version "%s".',
            version,
            cargo_version,
        )
        return 1


if __name__ == "__main__":
    sys.exit(main())
