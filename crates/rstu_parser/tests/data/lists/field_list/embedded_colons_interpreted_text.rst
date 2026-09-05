Edge cases involving embedded colons and interpreted text.

Recognized as field list items:

:field\:`name`: interpreted text (standard role) requires
                escaping a leading colon in a field name

:field:name: unambiguous, no need for escapes

:field::name: double colons are OK, too

:field:\`name`: not interpreted text

:`field name`:code:: interpreted text with role in the field name
                     works only when the role follows the text

:a `complex`:code:\  field name: field body

Not recognized as field list items:

::code:`not a field name`: paragraph with interpreted text

:code:`not a field name`: paragraph with interpreted text
