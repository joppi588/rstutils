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

# Remove username/arrow prefix from the prompt and render branch in light grey.
RUN python3 -c 'from pathlib import Path; theme = Path("/home/vscode/.oh-my-zsh/custom/themes/devcontainers.zsh-theme"); text = theme.read_text(); text = text.replace("    PROMPT=\"%{$fg[green]%}${prompt_username} %(?:%{$reset_color%}➜ :%{$fg_bold[red]%}➜ )\" # User/exit code arrow\\n", "    PROMPT=\"\" # No username or arrow prefix\\n"); text = text.replace("    PROMPT=\"%(?:%{$reset_color%}➜ :%{$fg_bold[red]%}➜ )\" # User/exit code arrow\\n", "    PROMPT=\"\" # No username or arrow prefix\\n"); text = text.replace("                echo -n \"%{$fg_bold[cyan]%}(%{$fg_bold[red]%}${BRANCH}\" \\\\n", "                echo -n \"%{$fg[8]%}(%{$fg[8]%}${BRANCH}\" \\\\n"); text = text.replace("                && echo -n \"%{$fg_bold[cyan]%})%{$reset_color%} \"; \\\\n", "                && echo -n \"%{$fg[8]%})%{$reset_color%} \"; \\\\n"); theme.write_text(text)'


# Pin Rust toolchain and include rustfmt so cargo fmt is available by default.
RUN rustup toolchain install 1.94 --profile minimal \
    && rustup component add rustfmt --toolchain 1.94 \
    && rustup component add rust-src --toolchain 1.94 \
    && rustup default 1.94

# Install cargo-nextest for aggregated Rust test execution and summaries.
RUN cargo install --locked cargo-nextest \
    && chown -R vscode:rustlang /usr/local/cargo
