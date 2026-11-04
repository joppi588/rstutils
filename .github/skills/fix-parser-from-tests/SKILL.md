---
name: fix-parser-from-tests
description: "Use when fixing Rust reStructuredText parser behavior from a failing test or fixture, including AST/YAML mismatches, ignored cases, and cases marked FIX in crates/rstu_parser/tests."
---

# Fix Parser From Tests

Use this workflow to turn a concrete failing parser test into a minimal parser fix. Treat the test input and expected output as the behavioral contract; do not make a failure disappear by weakening the assertion or changing the expected fixture without confirming the intended behavior.

## Procedure

1. Identify the exact test and fixture.
   - The user will provide the fixture name.
   - Read the test function, its `#[case]` entries, and the relevant helper in `crates/rstu_parser/tests/common/mod.rs`.
   - Find the `.rst` input and expected `.yaml` under `crates/rstu_parser/tests/data/`.
   - Hint: Check the pinned test file.

2. Identify the problem
   - Create a yaml fixture by running the rst2yaml script
   - Compare the created yaml file to the provided xml file in the same folder
   - The xml files are doctree, the yaml file are abstract syntax trees.
     Compare both, identify deviating elements.
   - Read docs/restructuredtext.rst for reference.

3. Trace the behavior to its owner.
   - Follow the failing input through the parser from its public entry point toward the code that directly recognizes or constructs the incorrect node.
   - Read the nearest related parser tests and implementation before changing code.
   - State a local hypothesis that explains the observed output and what result would disprove it.

4. Make the smallest parser change that tests the hypothesis.
   - Preserve the documented reStructuredText behavior and neighboring parser behavior.
   - Keep fixture changes separate from implementation changes unless the expected fixture is demonstrably wrong.
   - Do not uncomment unsupported cases, remove assertions, or broaden the change to unrelated parser features as a shortcut.

5. Verify in increasing scope.
   - Rerun the single failing fixture first.
   - Run the relevant test function or integration test to cover nearby cases.
   - Run `cargo test -p rstu_parser` after the focused checks pass.
   - If a check fails, use the new output to revise the local hypothesis, fix the same behavior, and rerun the narrow check.

## Completion Check

- The original active test passes without weakening its assertion.
- Relevant neighboring cases still pass.
- Expected fixtures were changed only when their prior output was confirmed incorrect.
- The `rstu_parser` package tests pass, or any remaining unrelated failures are reported clearly.
