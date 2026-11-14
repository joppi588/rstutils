This document lists deviations from the docutils parser implementation.

# Directives
rstu_parser does not support syntax extension via directives.
Rationale: Allowing arbitrary syntax extension is a powerful tool, but it prevents building a universal rst parser.
Directives content should follow rst syntax.
If post-processing is needed (e.g. type checking of arguments) this should be implemented
in a separate transform module, taking the abstract syntax tree as input.

Consequences:
- Directive bodies are parsed as sub-documents.
  New section hierarchy is started.

# Other parser behavior changes
All of these are believed to be non-critical. A few might be considered bug fixes.

- Allowed section/transition markers: restrict to the "recommended" set.
- Field names only allow [A-Za-z0-9_ ]
  Rationale: Field names are like identifiers.
- Bullet list:
  Indentation must align with the last paragraph
  (err/indented_bullet_list, TODO: tbc could be interpreted as a definition list)
- Block quotes:
  * Attribution markers must be followed by a space to avoid confusion with long options.
  * The parser treats attributions like other body elements.
    Checks regarding multiple / outside block quote / not ending a block quote are left to the
    subsequent processing tools (linter, renderer)
    Use empty comments (`..`) to explicitely split block quotes.
- Indentation: The first indent defines the block indent, not the smallest.
  Workaround: use empty comments
  see test cases outdenting_01 and outdenting_02.
- Line blocks:
  * First indent defines the block indent.
  * Representation: The first line is a direct child of the line block, even if nested line blocks follow.
- Inline markup
  * Asterisks-only surrounded emphasis markers need to be escaped (see emphasis_06.rst) to avoid confusion with section marker.
  * Substitution definitions without closing | are treated as ordinary paragraph and will not  cause an error.
  * All backquotes in interpreted text must be escaped (see interpreted/basics_03a,03b)
  * Escaped whitespace is treated as whitespace and not removed: e.g. inline_markup/embedded_aliases_07.rst


# Spec extensions
- Enumerated list:
    * Auto in between explicit markers is accepted.
      (test case docutils_mixed_auto_and_explicit, docutils does not accept that.)
- Transitions are 3+ characters, not 4+ as in the original spec.
  Rationale:
    * <BlankLine>===<BlankLine> would be interpreted by the parser as a section header with
      empty title and missing closing -> error
- whitespaces in uris are accepted, see tests\data\inline_markup\embedded_uris_07.rst


# Interpretations
Things that would be according to the spec book but not to the python parser implementation:
- Substitution definitions:
  * Missing :: or missing directive name are treated as a comments.
- character_level_inline_markup setting is not implemented and uses the default behavior.
  Test fixtures are included for completeness but not executed.


# Proposals
