# Architecture

## Functional/Logical

Parser - Syntactic analysis
Format checker - Semantic analysis and auto-correct
Reference checker
Transformation - Output
Language Server

# Design considerations
1. "Make or buy"
Option 1: Build on top of existing package.
Option 2: Start from Scratch
Start from scratch while reading the existing packages.
Rationale (afaik):
- rst_parser: Limitations of pest-parser approach (section stack, rst error detection);
- https://github.com/salioglu/sphinx-ultra: Monolithic, coupling with sphinx
- all: Doctree-based approaches contain a lot of interpretation already, beyond the pure syntax analysis. Not round-trip capable.

2. AST and Doctree
We use first an abstract syntax tree to parse the structure of the document (e.g. directives, indented blocks,...).
This step is beneficial since:
- the doctree according to docutils is already at a semantic level.
  e.g. a bullet list with mixed markers shall be allowed and eventually re-formatted automatically.
  The doctree requires a uniform marker already
- The AST shall allow round-trip (with minor limitations)
- AST focusses on pure syntax representation. Cross-Referencing, reference auto-numbering and validation is separated.
The transformation of AST into to a doctree is done by a later transformation step.


Implementation:
Option 1: Spezialized nodes for element types (inheritance-like)
Option 2: Generic node with an attribute for element type (composition)
Use composition.
Rationale: more simple AST definition

3. Lexer approach
Use a 1 character context before and after the token.

Drawback Information leakage:
The tokenizer and parser share information, e.g. the knowledge that a bullet list marker will be followed by a newline or space. This creates a dependency between the modules, if the lexer is updated, the parser needs to be checked as well. This dependency can sometimes be made explicit by parsing adjacent token pairs rather than single tokens.

4. Token stream
- In place modification introduces some kind of "state", which is usually an anti-pattern.  However, the cursor is only moving forward, and only unprocessed sections are modified. So data is used only once -> acceptable.

5. Parser approach
Top-down.
- Level 1: Document structure (sections), lookahead one line.
- Level 2: (recursive): Body elements
This represents the language structure.

6. Error handling while parsing
Error messages shall contain line and column number.
Option 1:
- One error type per Err statement
Option 2:
- One error type per parsing subfunction
- Further specialization via error messages
Decision: Option 1, error messages are created using the Display trait. Smaller overhead.

7. Warnings/Errors
Parser: Only emit errors, since syntax is ideally unambiguous
  Exception: Enumerated lists, the parser needs to decide if alphabet I (continue enum list) or roman 1 (start new list).
Format checker: Mainly warnings, with auto-correction proposals where feasible.
Reference checker: Errors for nonexisting targets, Warnings for likely mistyped targets.

Generally speaking, warnings represent interpretation + decisions, which are on a semantic level. Interpretation shall follow the principle of least surprise.


8. Blank lines and indentation:
- Blank lines before an indent are interpreted as indented.
  except after bullet list item, field list item, directives
- Blank lines before a dedent are interpreted as dedented.
So technically indented blocks can be surrounded by blank lines, and in/dedents treated upstream.

9. Format checker
Technically: Write back .rst from eventually modified AST.
TODO: Some elements contain the trailing newline, some not.
Make this consistent


# Architectural drivers
Development speed, especially bug fixes -> Maintainability is key
Execution speed
Easy installation -> low entry hurdle

# Assumptions
Preprocessing of text files / enforcement:
- All files have a trailing newline
- No trailing spaces at the end of the line
