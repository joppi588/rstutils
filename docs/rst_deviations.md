This document lists deviations from the rst specification / implementation.
All of these are believed to be non-critical.

# Spec limitations
- Allowed section/transition markers: restrict to the "recommended" set.
- Minimum length for section header marker: 4 chars
- character_level_inline_markup = False
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
  * Asterisks next to emphasis markers need to be escaped (see emphasis_06.rst)

# Spec extensions
- Enumerated list:
    * Auto in between explicit markers is accepted.
      (test case docutils_mixed_auto_and_explicit, docutils does not accept that.)

# Interpretations
Things that would be according to the spec book but not to the python parser implementation:
- (empty)

# Proposals
