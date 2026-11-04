# USE CASES

As a rst author,
- I want to get quick feedback (100ms) if my file is syntactically correct.
- I want to get quick feedback (1ms) if the internal links are working.
- I want to have syntax highlighting
- I want to have access to the link targets by e.g. clicking on them in my editor.
- I want auto-format such as automatic line breaks.

As a developer,
- I want to have all cross references and targets in a database so I can do a consistency check.

As a systems engineer,
- I want to use directives to model engineering artifacts.
- store these artifacts in a relational database and perform cross-link consistency checks.

# FEATURES

Current work focussing on parser, the rest are ideas only.

## Parser
Parse multiple rst files into a forest of abstract syntax trees (AST).
Parsing errors.
Round-trip capable

------------------------------------

## Transformers (using the AST)
- serialize to rst, json, yaml, html format.
- doctree transform
- database transforms
  * extract references
  * directives to relational db (fields and content as typed fields, indexed by an "id" field)

## Structure checks on AST
Check numbering of enumerations
Check url format

## Formatter (AST modification + rst serialisation)
Auto-correct indents, heading markups, table markups.
Auto-wrap
Remove empty lines
(replace tabs by spaces)

## Artifact server
- Language server protocol API support.
- References database
- directive database
- consistency checks, validation
- Parsing errors, warnings, linked artifacts

# CONSTRAINTS
- Cache
