# Format Warnings

Collection only - to be implemented in rstu-check

## General Warnings

- **Section marker shorter than title**
  - File: `section_headers`
  - Fix: Align with title length

- **Section marker length unbalanced** (if both over/under)
  - File: `section_headers`
  - Fix: Min possible (see above)

- **Empty section title**
  - File: `section_headers_14.rst`
  - Fix: n/a

- **Inconsistent section marker order or missing section level**
  - File: `section_headers_18.rst`
  - Fix: Use standard order / Use order defined first

- **Explicit markup ends without a blank line; unexpected unindent.**
  - Files: `comments_04`, `substitution_definitions_10`
  - Fix: Insert blank line

- **Empty citation**
  - File: `citations_05`
  - Fix: n/a

- **Missing blank line after various elements**
  - File: `indent_04b.rst`
  - Fix: insert blank line

- **Missing blank line before literal block**
  tbd, required by original spec?
  - File: `indented_03.rst`
  - Fix: insert blank line


## Directive Specific Warnings

- **Image directive does not allow content** (except blank line)
  - File: `substitution_definitions_05.rst`
  - Fix: Insert comment marker / dedent / insert empty comment

- **Duplicate substitutions**
  - File: `substitution_definitions_09.rst`
  - Fix: enumerate

- **References are not allowed as directive arguments.**
  - File: `substitution_definitions_11`
  - Fix: n/a
