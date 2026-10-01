# Architecture

## Logical

Parser - Syntactic analysis
Linter&Link checker - Semantic analysis
Formatter - Auto-correct
Transformation - Output
Language Server

# ADR
1. Programming Language
Rust for production
Rationale: ruff as reference implementation, cool language :)

2. "Make or buy"
Option 1: Build on top of rst_parser package
Option 2: Start from Scratch
Start from scratch while reading the existing packages.
Rationale: One of my goals was to learn Rust; limitations of pest-parser approach (section stack, rst error detection)

3. AST and Doctree
We use first an abstract syntax tree to parse the structure of the document (e.g. directives, indented blocks,...).
This step is beneficial since:
- the doctree according to docutils is already at a semantic level.
  e.g. a bullet list with mixed markers shall be allowed and eventually re-formatted automatically.
  The doctree requires a uniform marker already
- The AST shall allow round-trip (with minor limitations)
- AST focusses on pure syntax representation. Validation can be separated.
The transformation of AST into to a doctree is done by a later transformation step.

Implementation:
Option 1: Spezialized nodes for element types (inheritance-like)
Option 2: Generic node with an attribute for element type (composition)
Use composition.
Rationale: more simple AST definition

4. Tokenizer approach
Use a 1 character context before and after the token.

Drawback Information leakage:
The tokenizer and parser share information, e.g. the knowledge that a bullet list marker will be followed by a newline or space. This creates a dependency between the modules, if the lexer is updated, the parser needs to be checked as well. This dependency can sometimes be made explicit by parsing adjacent token pairs rather than single tokens.

5. Parser approach
Top-down.
- Level 1: Document structure (sections), lookahead one line.
- Level 2: Main blocks (directives, comments)
- Level 3: (recursive): Body elements
This represents the language structure.

6. Error handling while parsing
Error messages shall contain line and column number.
Option 1:
- One error type per Err statement
Option 2:
- One error type per parsing subfunction
- Further specialization via error messages
Decision: TODO (depends on error handling)

7. Warnings
Warning messages are emitted by the linter, the parser rather emits only errors.
Rationale: Syntax is ideally unambiguous.
Warnings represent interpretation, which is on a semantic level -> linter.
There are exceptions, e.g. the parser needs to decide if alphabet I (continue enum list) or roman 1 (start new list).

# Architectural drivers
Development speed, especially bug fixes -> Maintainability is key
Execution speed
Easy installation -> low entry hurdle

# Assumptions
Preprocessing of text files / enforcement:
- All files have a trailing newline
- No trailing spaces at the end of the line
