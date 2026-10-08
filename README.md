Tools to process RST files

- Parser
- Linter
- Formatter
- Language Server

Written in rust

Standing on the shoulders of giants:
- ruff
- docutils

## Releasing

The crates.io release includes `rstu_ast`, `rstu_parser`, and `rstu`. The
`rstu_doctree` crate is workspace-only and is not published.

For the initial release, configure a crates.io token with `cargo login`, then
run:

```sh
scripts/publish-crates.sh --dry-run
scripts/publish-crates.sh
```

For later releases, bump all workspace crate versions together, review and
commit the resulting manifest and lockfile changes, then publish:

```sh
scripts/bump-version.sh 0.0.2
cargo test --workspace
scripts/publish-crates.sh
```

The bump script updates the workspace version and the internal crate version
requirements. The publish script tests the workspace and publishes crates in
dependency order.


What does "rstu" stand for?

Pick one:
- "Uh, u was just the next letter in the alphabet."
- "Rust was already taken, so I permuted the letters."
- "RST Utils."
