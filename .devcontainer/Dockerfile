FROM mcr.microsoft.com/devcontainers/rust:1

# Install REUSE CLI for SPDX/license compliance workflows.
RUN apt-get update \
    && apt-get install -y --no-install-recommends python3-pip reuse \
    && rm -rf /var/lib/apt/lists/*

RUN python3 -m pip install --break-system-packages --no-cache-dir pre-commit


# Persist zsh history in a mounted volume across container rebuilds.
RUN mkdir -p /commandhistory \
    && chown vscode:rustlang /commandhistory \
    && printf '\nHISTFILE=/commandhistory/.zsh_history\nHISTSIZE=20000\nSAVEHIST=20000\nsetopt APPEND_HISTORY\nsetopt INC_APPEND_HISTORY\nsetopt SHARE_HISTORY\nsetopt HIST_IGNORE_DUPS\nsetopt HIST_IGNORE_SPACE\nsetopt HIST_REDUCE_BLANKS\n' >> /home/vscode/.zshrc \
    && chown vscode:rustlang /home/vscode/.zshrc

# Add gray inline history suggestions and allow accepting them with the right arrow.
RUN git clone --depth 1 https://github.com/zsh-users/zsh-autosuggestions \
        /home/vscode/.oh-my-zsh/custom/plugins/zsh-autosuggestions \
    && printf '\nsource $HOME/.oh-my-zsh/custom/plugins/zsh-autosuggestions/zsh-autosuggestions.zsh\nZSH_AUTOSUGGEST_HIGHLIGHT_STYLE="fg=8"\nbindkey "^[[C" autosuggest-accept\nbindkey "^[OC" autosuggest-accept\n' >> /home/vscode/.zshrc \
    && chown -R vscode:rustlang /home/vscode/.oh-my-zsh/custom /home/vscode/.zshrc


# Pin Rust toolchain and include rustfmt so cargo fmt is available by default.
RUN rustup toolchain install 1.94 --profile minimal \
    && rustup component add rustfmt --toolchain 1.94 \
    && rustup component add rust-src --toolchain 1.94 \
    && rustup default 1.94

# Install cargo-nextest for aggregated Rust test execution and summaries.
RUN cargo install --locked cargo-nextest \
    && chown -R vscode:rustlang /usr/local/cargo
