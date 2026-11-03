# Features

Current work focussing on parser, the rest are ideas only.

## Parser
Parse multiple rst files into a forest of abstract syntax trees.
Output in json and yaml format.
Parsing errors.
Round-trip capable

------------------------------------

## Transformers
- Syntax highlighting
- doctree transform
- html transform

## Semantic checks
Check numbering of enumerations
Validate the doctree against a schema.
    validate data, e.g. numbers, units
Check url format

## Formatter
Auto-correct indents, heading markups, table markups.
Auto-wrap
Remove empty lines
(replace tabs by spaces)

## Artifact server
- Link database
- directive database (fields and content as typed fields, indexed by an "id" field)
- consistency checks

## Language server
Parsing errors.
Linter warnings
Check internal & external links

# Constraints
- Cache
