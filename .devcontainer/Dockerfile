FROM mcr.microsoft.com/vscode/devcontainers/base:bullseye

ENV DENO_INSTALL="/usr/local"
ENV RUSTUP_HOME="/usr/local"
ENV CARGO_HOME="/usr/local"

# Make sure repositories are up to date
RUN ["/usr/bin/apt", "update", "-y"]
RUN ["/usr/bin/apt", "upgrade", "-y"]

# Give vscode user permissions to /usr/local
RUN ["/bin/chown", "-R", "root:vscode", "/usr/local"]
RUN ["/bin/chmod", "-R", "775", "/usr/local"]

# Install base dependencies. TODO: figure out why installing npm takes > 5 minutes
RUN ["/usr/bin/apt", "install", "-y", "nodejs"]
RUN ["/usr/bin/apt", "install", "-y", "npm"]
RUN ["/usr/bin/apt", "install", "-y", "build-essential"]
RUN ["/usr/bin/sudo", "-Eu", "vscode", "sh", "-c", "/usr/bin/curl -fsSL https://deno.land/install.sh | sh"]
RUN ["/usr/bin/sudo", "-Eu", "vscode", "sh", "-c", "/usr/bin/curl -fsSL https://sh.rustup.rs | sh -s -- --no-modify-path -y -c rls"]

# Install & enable corepack (for Yarn)
RUN ["/usr/bin/npm", "install", "--global", "corepack"]
RUN ["/usr/local/bin/corepack", "enable"]

# Install some other CLIs
RUN ["/usr/bin/npm", "install", "--global", "nx"]
RUN ["/usr/bin/npm", "install", "--global", "prettier"]
RUN ["/usr/bin/npm", "install", "--global", "typescript"]
RUN ["/usr/bin/sudo", "-iu", "vscode", "/usr/local/bin/cargo", "install", "--locked", "wasm-pack"]
