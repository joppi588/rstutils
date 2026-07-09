FROM mcr.microsoft.com/devcontainers/rust:1

# Install REUSE CLI for SPDX/license compliance workflows.
RUN apt-get update \
    && apt-get install -y --no-install-recommends reuse \
    && rm -rf /var/lib/apt/lists/*

# Install cargo-nextest for aggregated Rust test execution and summaries.
RUN cargo install --locked cargo-nextest \
    && chown -R vscode:rustlang /usr/local/cargo
