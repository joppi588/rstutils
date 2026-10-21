some punctuation is allowed around inline markup, e.g.
/*emphasis*/, -*emphasis*-, and :*emphasis*: (delimiters),
(*emphasis*), [*emphasis*], <*emphasis*>, {*emphasis*} (open/close pairs)
*emphasis*., *emphasis*,, *emphasis*!, and *emphasis*\ (closing delimiters),

With simple-inline-markup also
)*emphasis*(, ]*emphasis*[, >*emphasis*>, }*emphasis*{ (close/open pairs),
x*2* or 2*x* (alphanumeric char before),

but not
(*), [*], '*' or '"*"' ("quoted" start-string),
\*args or * (escaped; whitespace behind start-string),
or *the\* *stars\* *inside* (escaped; whitespace before end-string).

However, '*args' triggers a warning.

Also *this**.
