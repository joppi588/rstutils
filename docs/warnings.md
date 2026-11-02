# Format Warnings

Collection only - to be implemented in rstu-check

---
Section marker shorter than title
section_headers.
Align with title length
---
Section marker length unbalanced (if both over/under)
section_headers
Min possible (see above)
---
Empty section title
section_headers_14.rst
n/a
---
Inconsistent section marker order or missing section level
section_headers_18.rst
Use standard order / Use order defined first
---
Explicit markup ends without a blank line; unexpected unindent.
comments_04, substitution_definitions_10
Insert blank line
---
Empty citation
citations_05
n/a


# Directive specific warnings

image directive does not allow content (except blank line)
substitution_definitions_05.rst
Insert comment marker / dedent / insert empty comment
---
duplicate substitutions
substitution_definitions_09.rst
enumerate
---
References are not allowed as directive arguments.
substitution_definitions_11
n/a
