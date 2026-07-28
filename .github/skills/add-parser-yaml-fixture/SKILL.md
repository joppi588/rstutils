---
name: add-parser-yaml-fixture
description: "Use when creating or updating rstu_parser YAML fixtures that represent expected AST output for .rst test cases."
---

# Add Parser YAML Fixture

Use this skill when asked to create an expected YAML tree fixture for an `.rst` test file in `crates/rstu_parser/tests/data`.

## Process

1. Locate the source `.rst` fixture and identify its test area.
   - Keep section fixtures in `tests/data/sections`.
   - Keep body fixtures in `tests/data/body`.

2. Derive the expected AST from current parser behavior.
   - Read `crates/rstu_parser/src/lib.rs` to confirm what `parse()` currently handles.
   - Do not model unsupported nodes yet; expected YAML must match current implementation.

3. Build the YAML with stable shape.
   - Root must be:
     - `kind: Document`
     - `attributes: {}`
     - `text: null`
     - `children: [...]`
   - Match `AstNode::to_yaml()` structure exactly:
     - `kind`, `attributes`, `text`, `children`
   - Preserve newlines in title text (for example `"Title\n"`).

4. Name and place the fixture consistently.
   - Use the same basename as the `.rst` file.
   - Example: `ok_strong.rst` -> `ok_strong.yaml`.

5. Validate fixture semantics.
   - Ensure YAML parses as valid `serde_yaml::Value`.
   - Keep values minimal and deterministic.

## Optional Verification

If a matching parser integration test exists, run targeted tests first:

- `cargo test -p rstu_parser --test test_parser_section`

If body-parser tests are added later, run those targeted tests as well.

## Completion Check

The task is complete when:
- The YAML fixture exists at the expected path.
- The YAML structure matches current parser output format.
- File naming aligns with the source `.rst` fixture.
