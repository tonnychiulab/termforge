#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""Fail a PR when gated paths change without a complete 基線條款適用表."""

from __future__ import annotations

import os
import re
import subprocess
import sys
from pathlib import Path

CLAUSES = (
    "信任邊界與誠實能力",
    "託管與發布路徑一致",
    "第三方腳本不得無完整性載入",
    "平台做不到的控制不得當成已生效",
    "機密不得進倉",
    "輸入與資源有界",
    "不適用要寫下來",
)

GRANDFATHERED = frozenset(
    {
        "termforge-engine",
        "add-web-wasm-support",
        "enhanced-controls-and-abilities",
        "add-in-page-controls-cheat-sheet",
    }
)

GATED_PREFIXES = (
    ".github/",
    ".cursor/",
    "openspec/specs/",
    "scripts/",
    "dist/",
    "termforge/",
    "termforge_web/",
    "asteroids/",
)
GATED_NAMES = frozenset(
    {
        "openspec/config.yaml",
        "Cargo.toml",
        "Cargo.lock",
    }
)
GATED_SUFFIXES = (".rs",)

ROOT = Path(__file__).resolve().parents[1]


def posix(path: str) -> str:
    return path.replace("\\", "/")


def is_gated(path: str) -> bool:
    path = posix(path)
    if path in GATED_NAMES or path.endswith("/Cargo.toml") or path.endswith("/Cargo.lock"):
        return True
    if path.endswith(GATED_SUFFIXES):
        return True
    return any(path.startswith(prefix) for prefix in GATED_PREFIXES)


def change_id_from_path(path: str) -> str | None:
    parts = posix(path).split("/")
    if len(parts) >= 3 and parts[0] == "openspec" and parts[1] == "changes":
        return parts[2]
    return None


def table_errors(markdown: str) -> list[str]:
    errors: list[str] = []
    for clause in CLAUSES:
        pattern = (
            r"\|\s*"
            + re.escape(clause)
            + r"\s*\|\s*(適用|不適用)\s*\|\s*\S"
        )
        if re.search(pattern, markdown) is None:
            errors.append(f"missing applicability row: {clause}")
    return errors


def read_text(path: Path) -> str:
    return path.read_text(encoding="utf-8")


def check_change_dir(change_dir: Path) -> list[str]:
    blob = ""
    for name in ("design.md", "proposal.md"):
        candidate = change_dir / name
        if candidate.is_file():
            blob += read_text(candidate) + "\n"
    if not blob.strip():
        return [f"{change_dir.name}: no design.md or proposal.md"]
    return [f"{change_dir.name}: {err}" for err in table_errors(blob)]


def list_change_dirs(root: Path) -> list[Path]:
    changes = root / "openspec" / "changes"
    if not changes.is_dir():
        return []
    return sorted(p for p in changes.iterdir() if p.is_dir())


def evaluate(
    *,
    root: Path,
    changed_files: list[str] | None,
) -> list[str]:
    errors: list[str] = []
    for change_dir in list_change_dirs(root):
        if change_dir.name in GRANDFATHERED:
            continue
        errors.extend(check_change_dir(change_dir))

    if changed_files is None:
        return errors

    gated = [posix(p) for p in changed_files if is_gated(p)]
    touched_ids = {
        cid
        for cid in (change_id_from_path(p) for p in changed_files)
        if cid and cid not in GRANDFATHERED
    }
    if gated and not touched_ids:
        errors.append(
            "gated paths changed without a non-grandfathered OpenSpec change "
            f"in the same diff: {', '.join(gated)}"
        )
    return errors


def git_changed_files(root: Path, base: str) -> list[str]:
    result = subprocess.run(
        ["git", "diff", "--name-only", f"{base}...HEAD"],
        cwd=root,
        check=True,
        capture_output=True,
        text=True,
    )
    return [line.strip() for line in result.stdout.splitlines() if line.strip()]


def self_test() -> int:
    failures: list[str] = []

    complete = "\n".join(
        f"| {clause} | 適用 | reason |" for clause in CLAUSES
    )
    if table_errors(complete):
        failures.append("complete table should pass")

    missing = complete.replace("| 第三方腳本不得無完整性載入 | 適用 | reason |\n", "")
    if "missing applicability row: 第三方腳本不得無完整性載入" not in table_errors(missing):
        failures.append("missing CDN clause should fail")

    if is_gated("asteroids/src/main.rs") is False:
        failures.append("rust files must be gated")
    if is_gated("docs/WORKLOG_2026-09-17.md") is True:
        failures.append("docs must not be gated")
    if is_gated("README.md") is True:
        failures.append("README must not be gated")
    if is_gated(".github/workflows/applicability.yml") is False:
        failures.append("workflows must be gated")

    from tempfile import TemporaryDirectory

    with TemporaryDirectory() as tmp:
        root = Path(tmp)
        changes = root / "openspec" / "changes"
        (changes / "termforge-engine").mkdir(parents=True)
        (changes / "termforge-engine" / "design.md").write_text(
            "no table here\n", encoding="utf-8"
        )
        current = changes / "add-example"
        current.mkdir()
        (current / "design.md").write_text(complete, encoding="utf-8")

        tree_errors = evaluate(root=root, changed_files=None)
        if tree_errors:
            failures.append(f"grandfather + complete current should pass: {tree_errors}")

        docs_only = evaluate(root=root, changed_files=["docs/WORKLOG.md", "README.md"])
        if docs_only:
            failures.append(f"docs-only should pass: {docs_only}")

        gated_no_change = evaluate(root=root, changed_files=["asteroids/src/main.rs"])
        if not any("gated paths changed" in err for err in gated_no_change):
            failures.append(f"rust without change should fail: {gated_no_change}")

        gated_with_change = evaluate(
            root=root,
            changed_files=[
                "asteroids/src/main.rs",
                "openspec/changes/add-example/design.md",
            ],
        )
        if gated_with_change:
            failures.append(f"rust + table should pass: {gated_with_change}")

        (current / "design.md").write_text(missing, encoding="utf-8")
        incomplete = evaluate(root=root, changed_files=None)
        if not any("第三方腳本不得無完整性載入" in err for err in incomplete):
            failures.append(f"incomplete current table should fail: {incomplete}")

    if failures:
        for item in failures:
            print(f"SELF-TEST FAIL {item}", file=sys.stderr)
        return 1
    print("SELF-TEST PASS")
    return 0


def main(argv: list[str]) -> int:
    if "--self-test" in argv:
        return self_test()

    base = None
    skip_diff = False
    args = list(argv)
    while args:
        arg = args.pop(0)
        if arg == "--base":
            base = args.pop(0) if args else None
        elif arg == "--skip-diff":
            skip_diff = True
        else:
            print(f"unknown argument: {arg}", file=sys.stderr)
            return 2

    if base is None and not skip_diff:
        env_base = os.environ.get("GITHUB_BASE_REF", "").strip()
        if env_base:
            base = f"origin/{env_base}"

    changed: list[str] | None
    if skip_diff:
        changed = None
    elif base:
        changed = git_changed_files(ROOT, base)
    else:
        changed = None

    errors = evaluate(root=ROOT, changed_files=changed)
    if errors:
        for item in errors:
            print(f"FAIL {item}", file=sys.stderr)
        return 1
    print("PASS secure-by-construction applicability")
    return 0


if __name__ == "__main__":
    sys.exit(main(sys.argv[1:]))
