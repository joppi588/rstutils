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
TESTS_DIR = ROOT / "crates" / "rstu_parser" / "tests"


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
    args = parser.parse_args()

    topic =  args.test_file.stem.removeprefix("test_")
    totest = load_totest(args.test_file.resolve())

    cases: dict[str, list[str]] = {"ok": [], "err": []}
    for name, group in totest.items():
        for index, case in enumerate(group):
            source, expected = case[0], case[1]
            kind = "err" if "<system_message" in expected else "ok"
            case_name = f"{name}_{index:02d}"
            path = {suffix: TESTS_DIR / "data" / topic / kind / f"{case_name}.{suffix}" for suffix in ["rst","xml"]}
            cases[kind].append(case_name)
            path["rst"].parent.mkdir(parents=True, exist_ok=True)
            path["rst"].write_text(source, encoding="utf-8")
            path["xml"].write_text(expected, encoding="utf-8")

    test_file = TESTS_DIR / f"test_parser_{topic}.rs"
    if not test_file.exists():
        test_file.parent.mkdir(parents=True, exist_ok=True)
    test_file.write_text(render_test(topic, cases), encoding="utf-8")


if __name__ == "__main__":
    sys.exit(main())
