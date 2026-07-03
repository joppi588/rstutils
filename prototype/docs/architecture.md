# Architecture

## Logical

Parser
Linter
Formatter
Language Server

# ADR

1. Prototype
Build a prototype for a parser in python.
Rationale: Quick setup, familiar language

2. Programming Language
Rust for production -> ruff as reference implementation

# Architectural drivers
Development speed, especially bug fixes -> Maintainability is key
Execution speed