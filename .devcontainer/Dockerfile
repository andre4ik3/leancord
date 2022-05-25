FROM mcr.microsoft.com/vscode/devcontainers/base:bullseye

ENV DENO_DIR="/usr/local/deno"
ENV DENO_INSTALL="/usr/local"
ENV RUSTUP_HOME="/usr/local"
ENV CARGO_HOME="/usr/local"

# Make sure repositories are up to date
RUN ["/usr/bin/apt", "update", "-y"]
RUN ["/usr/bin/apt", "upgrade", "-y"]

# Give vscode user permissions to /usr/local
RUN ["/bin/mkdir", "/usr/local/deno"]
RUN ["/bin/chown", "-R", "root:vscode", "/usr/local", "/usr/local/deno"]
RUN ["/bin/chmod", "-R", "775", "/usr/local", "/usr/local/deno"]

# Install base dependencies.
RUN ["/bin/bash", "-c", "curl -fsSL https://deb.nodesource.com/setup_lts.x | bash -"]
RUN ["/usr/bin/sudo", "-Eu", "vscode", "sh", "-c", "/usr/bin/curl -fsSL https://deno.land/install.sh | sh"]
RUN ["/usr/bin/sudo", "-Eu", "vscode", "sh", "-c", "/usr/bin/curl -fsSL https://sh.rustup.rs | sh -s -- --no-modify-path -y -t wasm32-unknown-unknown -c rls"]
RUN ["/usr/bin/apt", "install", "-y", "nodejs"]

# Prepare Yarn
RUN ["/usr/bin/corepack", "enable"]

# Install some other CLIs
RUN ["/usr/bin/npm", "install", "--global", "nx"]
RUN ["/usr/bin/npm", "install", "--global", "prettier"]
RUN ["/usr/bin/npm", "install", "--global", "typescript"]
RUN ["/usr/bin/sudo", "-Eu", "vscode", "sh", "-c", "deno install -Afq -n deno_bindgen https://deno.land/x/deno_bindgen/cli.ts"]
RUN ["/usr/bin/sudo", "--preserve-env=RUSTUP_HOME,CARGO_HOME", "-u", "vscode", "/usr/local/bin/cargo", "install", "--locked", "wasm-pack"]
