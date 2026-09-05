Some edge cases:

:Empty:
:Author: Me
No blank line before this paragraph.

: Field: marker must not begin with whitespace.

:Field : marker must not end with whitespace.

Field: marker is missing its open-colon.

:Field marker is missing its close-colon.

:Field\: names\: with\: colons\:: are possible.

:\\Field\  names with backslashes\\: are possible, too.

:\\: A backslash.

:Not a\\\: field list.

:Not a \: field list either.

:\: Not a field list either.

:\:
    A definition list, not a field list.
