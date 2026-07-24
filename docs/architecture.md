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

3. AST
Option 1: Spezialized nodes for element types (inheritance-like)
Option 2: Generic node with an attribute for element type (composition)
Use composition.
Rationale: more simple AST definition

4. Tokenizer approach
tbd (granularity of tokens)

5. Parser approach
tbd (bottom-up vs top-down)
Will be some hybrid solution.


# Architectural drivers
Development speed, especially bug fixes -> Maintainability is key
Execution speed
Easy installation -> low entry hurdle
