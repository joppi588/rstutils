---
name: convert-docutils-tests
description: Convert docutils tests
---

# Convert docutils tests

Use this skill when converting docutils tests.
Docutils tests are written in python.
They contain hard-coded rst snippets and the expected parsing result.

For each test file, create two new rust test functions:
    Expected error: If the parsing result has a system_message, create a test case expecting an error
    Ok: Otherwise assert that the parsing result mathes the yaml fixture
For each test case in the test file, create rst and yaml fixtures, and a test case for rstest.
Delete test functions without associated test cases.
