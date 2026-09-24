#!/usr/bin/env python3

from __future__ import annotations

import re
from pathlib import Path

REPO_ROOT = Path(__file__).resolve().parent.parent
EXAMPLES_DIR = REPO_ROOT / "codesamples" / "examples"
SRC_DIR = REPO_ROOT / "src"


def read_markdown_references() -> set[str]:
    references: set[str] = set()

    for markdown_file in sorted(SRC_DIR.glob("*.md")):
        text = markdown_file.read_text(encoding="utf-8")

        references.update(
            re.findall(
                r"\{\{#include\s+[^\n]*?/codesamples/examples/([A-Za-z0-9_./-]+)",
                text,
            )
        )
        references.update(
            re.findall(
                r"screenshots/([A-Za-z0-9_./-]+)\.(?:svg|png|jpg|jpeg|gif)",
                text,
            )
        )

    return references


def normalize_reference(reference: str) -> str | None:
    name = Path(reference).name

    if name.endswith(".rs"):
        return name

    if name.endswith((".svg", ".png", ".jpg", ".jpeg", ".gif")):
        stem = Path(name).stem
        example_path = EXAMPLES_DIR / f"{stem}.rs"
        if example_path.exists():
            return example_path.name
        return stem

    example_path = EXAMPLES_DIR / f"{name}.rs"
    if example_path.exists():
        return example_path.name

    if name in {path.name for path in EXAMPLES_DIR.glob("*.rs")}:
        return name

    return None


def main() -> int:
    all_examples = {path.name for path in EXAMPLES_DIR.glob("*.rs")}
    used_examples: set[str] = set()

    for reference in read_markdown_references():
        normalized = normalize_reference(reference)
        if normalized is not None:
            used_examples.add(normalized)

    unused_examples = sorted(all_examples - used_examples)

    if unused_examples:
        print("Unused example files found:")
        for example in unused_examples:
            print(f"  - {example}")
        print()
        print("Either include them in the book or remove them from the examples directory.")
        return 1

    print(f"All {len(all_examples)} example files are referenced in the book.")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
