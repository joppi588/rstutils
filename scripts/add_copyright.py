from __future__ import annotations

import sys
from pathlib import Path


ROOT = Path(__file__).resolve().parent.parent
COPYRIGHT_FILE = ROOT / "COPYRIGHT"
COMMENT_MARKERS = {".rs":"//",".py":"#"}


def comment_out_header(raw_header: str, file_type:str) -> str:
    return "\n".join([f"{COMMENT_MARKERS[file_type]} {line}" for line in raw_header.splitlines()])

def main(argv: list[str]) -> int:
    changed = False

    for file_name in argv[1:]:
        path = Path(file_name)
        if not path.is_file():
            continue

        header = comment_out_header(COPYRIGHT_FILE.read_text(encoding="utf-8"),path.suffix)

        original = path.read_text(encoding="utf-8")
        if original.startswith(header):
            continue

        first_lines = "\n".join(original.splitlines()[:5])
        if "SPDX-FileCopyrightText:" in first_lines or "SPDX-License-Identifier:" in first_lines:
            continue

        path.write_text(f"{header}\n{original}", encoding="utf-8")
        print(f"added copyright header to {path}")
        changed = True

    return 1 if changed else 0


if __name__ == "__main__":
    raise SystemExit(main(sys.argv))