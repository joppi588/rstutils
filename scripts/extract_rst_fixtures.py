# SPDX-FileCopyrightText: 2026 Jochen Schmaehling <tostmann1@web.de>
#
# SPDX-License-Identifier: MIT
"""Extract rst fixtures from the `totest` dict of a docutils parser test."""
from __future__ import annotations

import argparse
import importlib.util
import re
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
DOCUTILS_ROOT = ROOT / "docutils"
DEFAULT_OUT = ROOT / "crates" / "rstu_parser" / "tests" / "data"
DEFAULT_TEST_OUT = ROOT / "crates" / "rstu_parser" / "tests"


def load_totest(test_file: Path) -> dict:
    sys.path.insert(0, str(DOCUTILS_ROOT))
    spec = importlib.util.spec_from_file_location(test_file.stem, test_file)
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module.totest


def render_test(topic: str, cases: dict[str, list[str]]) -> str:
    topic_id = re.sub(r"\W", "_", topic)
    if not topic_id or topic_id[0].isdigit():
        topic_id = f"topic_{topic_id}"

    lines = [
        "// SPDX-FileCopyrightText: 2026 Jochen Schmaehling <tostmann1@web.de>",
        "//",
        "// SPDX-License-Identifier: MIT",
        "",
        "use rstest::rstest;",
        "",
        '[path = "common/mod.rs"]',
        "mod test_parser;",
        "use test_parser::rst_err;",
        "use test_parser::rst_vs_yaml;",
    ]
    for kind, macro in (("ok", "rst_vs_yaml"), ("err", "rst_err")):
        names = cases[kind]
        if not names:
            continue
        lines.extend(["", "#[rstest]"])
        lines.extend(f'#[case("{name}")]' for name in names)
        function = f"ok_{topic_id}" if kind == "ok" else f"err_{topic_id}_diagnostics"
        lines.extend([
            f"fn {function}(#[case] test_case: &str) {{",
            f'    {macro}!("{topic}/{kind}", test_case);',
            "}",
        ])
    return "\n".join(lines) + "\n"


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("test_file", type=Path)
    parser.add_argument("--topic", help="fixture directory (default: test file name without 'test_')")
    parser.add_argument("--out", type=Path, default=DEFAULT_OUT)
    parser.add_argument("--test-out", type=Path, default=DEFAULT_TEST_OUT)
    parser.add_argument("--dry-run", action="store_true")
    parser.add_argument("--force", action="store_true", help="overwrite existing fixtures")
    args = parser.parse_args()

    topic = args.topic or args.test_file.stem.removeprefix("test_")
    totest = load_totest(args.test_file.resolve())

    found = written = 0
    cases: dict[str, list[str]] = {"ok": [], "err": []}
    skipped = []
    for name, group in totest.items():
        for index, case in enumerate(group):
            found += 1
            source, expected = case[0], case[1]
            kind = "err" if "<system_message" in expected else "ok"
            case_name = f"{name}_{index:02d}"
            path = {suffix: args.out / topic / kind / f"{case_name}.{suffix}" for suffix in ["rst","xml"]}
            cases[kind].append(case_name)
            if path["rst"].exists() and not args.force:
                skipped.append(path)
                continue
            if not args.dry_run:
                path["rst"].parent.mkdir(parents=True, exist_ok=True)
                path["rst"].write_text(source, encoding="utf-8")
                path["xml"].write_text(expected, encoding="utf-8")
            written += 1

    test_file = args.test_out / f"test_parser_{topic}.rs"
    if not args.dry_run and (args.force or not test_file.exists()):
        test_file.parent.mkdir(parents=True, exist_ok=True)
        test_file.write_text(render_test(topic, cases), encoding="utf-8")

    return 0 if written + len(skipped) == found else 1


if __name__ == "__main__":
    sys.exit(main())
