# Architecture

## Logical

Parser
Linter
Formatter
Language Server

# ADR
1. Programming Language
Rust for production
Rationale: ruff as reference implementation, cool language :)

2. Make or buy
Option 1: Build on top of rst_parser package
Option 2: Start from ScratchAst
Start from scratch while reading the existing packages
Rationale: Limited rust knowledge at project start, limitations of pest-parser approach (section stack, rst error detection)

3. AST and Doctree
We use first an abstract syntax tree to parse the structure of the document (e.g. directives, indented blocks,...).
This step is beneficial since the doctree according to docutils is already at a semantic level.
The transformation of AST into to a doctree is done by the linter.

Implementation:
Option 1: Spezialized nodes for element types (inheritance-like)
Option 2: Generic node with an attribute for element type (composition)
Use composition.
Rationale: more simple AST definition

4. Tokenizer approach
Use a 1 character context before and after the token.

5. Parser approach
Top-down.
- Level 1: Document structure (sections)
           Lookahead one line.
- Level 2: Main blocks (directives, comments)
- Level 3 (recursive): Body elements
This represents the language structure.

# Architectural drivers
Development speed, especially bug fixes -> Maintainability is key
Execution speed
Easy installation -> low entry hurdle
