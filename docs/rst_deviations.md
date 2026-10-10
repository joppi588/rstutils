This document lists deviations from the rst specification / implementation.

# Spec deviations
- Allowed section/transition markers: restrict to the "recommended" set.
- Minimum length for section header marker: 4 chars (tbc)
- character_level_inline_markup = False
- Field names only allow A-Za-z0-9_
  Rationale: Field names are like identifiers.
- Bullet list:
  Indentation must align with the last paragraph
  (err/indented_bullet_list, tbc could be interpreted as a definition list)
- Enumerated list:
    * Auto in between explicit markers is accepted.
      (test case docutils_mixed_auto_and_explicit, docutils does not accept that.)

# Interpretations
Things that would be according to the spec book but not to the python parser implementation:
- Attribution markers must be followed by a space to avoid confusion with long options.

# Proposals
