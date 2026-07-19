FROM mcr.microsoft.com/devcontainers/rust:1

# Install REUSE CLI for SPDX/license compliance workflows.
RUN apt-get update \
    && apt-get install -y --no-install-recommends python3-pip reuse \
    && rm -rf /var/lib/apt/lists/*

RUN python3 -m pip install --break-system-packages --no-cache-dir pre-commit


# Pin Rust toolchain and include rustfmt so cargo fmt is available by default.
RUN rustup toolchain install 1.94 --profile minimal \
    && rustup component add rustfmt --toolchain 1.94 \
    && rustup component add rust-src --toolchain 1.94 \
    && rustup default 1.94

# Install cargo-nextest for aggregated Rust test execution and summaries.
RUN cargo install --locked cargo-nextest \
    && chown -R vscode:rustlang /usr/local/cargo
