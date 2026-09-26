This document lists deviations from the rst specification.

# Deviations
- Allowed section/transition markers: restrict to the "recommended" set.
- Minimum length for section header marker: 4 chars (tbc)
- character_level_inline_markup = False
- Field names only allow A-Za-z0-9_ (Why? Field names are like identifiers)
- Bullet list: Indentation must align with the last paragraph.


# Interpretations

# Proposals
1. Remove "Fully minimized form" for Literal blocks.
   Rationale:
   - Hard to read
   - no large benefit over partially minimized form
   - special case, extra parsing effort
