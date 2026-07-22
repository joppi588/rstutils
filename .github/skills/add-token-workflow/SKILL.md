---
name: add-token-workflow
description: "Use when adding a new lexer token in rstu_parser: update TokenKind, add a three-part regex with 1-char pre-context and post-context, wire dispatch/order, add unit tests, and verify the lexer output."
---

# Add Token Workflow

Use this skill when adding a new token backed by a regular expression in `rstu_parser`.

## Process

1. Inspect the existing token model.
   - Find `TokenKind` in `crates/rstu_parser/src/token.rs`.
   - Check the `ALL` ordering, `name()`, and `regex()` dispatch.
   - Review existing tests for similar token kinds.

2. Define the regex.
   - Use three matching groups: pre-context, real match, and post-context.
   - Keep the captured token in group 1.
   - Make the pre-context and post-context exactly one character wide when the token model expects one-character context.
   - Put the new token kind before the fallback `LiteralChar` entry if it should win over fallback matching.

3. Add the token kind.
   - Add the new variant to `TokenKind`.
   - Extend `TokenKind::ALL`.
   - Extend `TokenKind::name()`.
   - Extend `TokenKind::regex()`.

4. Add tests.
   - Add a focused unit test for `TokenKind::is_match()`.
   - Add a negative test that proves the regex does not overmatch.
   - If the token should appear in the main lexer stream, update the integration fixture or add a new lexer test.

5. Verify.
   - Run the narrowest lexer or token test first.
   - If it passes, run the package test suite for `rstu_parser`.
   - Confirm the new token does not break adjacent token kinds.

## Completion Check

The task is complete when:
- The new token kind is exposed in `TokenKind`.
- The regex matches the intended text with the correct context boundaries.
- Tests cover the positive and negative cases.
- The lexer output includes the new token where expected.
- Focused and package-level tests pass.
