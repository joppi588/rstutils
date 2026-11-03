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
- database transforms
  * extract references
  * directives to relational db (fields and content as typed fields, indexed by an "id" field)

## Structure checks
Check numbering of enumerations
Check url format

## Formatter
Auto-correct indents, heading markups, table markups.
Auto-wrap
Remove empty lines
(replace tabs by spaces)

## Artifact server
- References database
- directive database
- consistency checks, validation

## Language server
Parsing errors, warnings, linked artifacts

# Constraints
- Cache
