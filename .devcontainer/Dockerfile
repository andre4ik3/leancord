FROM mcr.microsoft.com/vscode/devcontainers/base:bullseye

ENV DENO_INSTALL="/usr/local"
ENV RUSTUP_HOME="/usr/local"
ENV CARGO_HOME="/usr/local"

# Make sure repositories are up to date
RUN ["/usr/bin/apt", "update", "-y"]
RUN ["/usr/bin/apt", "upgrade", "-y"]

# Install base dependencies. TODO: figure out why installing npm takes > 5 minutes
RUN ["/usr/bin/apt", "install", "-y", "nodejs"]
RUN ["/usr/bin/apt", "install", "-y", "npm"]
RUN ["/usr/bin/apt", "install", "-y", "build-essential"]
RUN ["sh", "-c", "/usr/bin/curl -fsSL https://deno.land/install.sh | sh"]
RUN ["sh", "-c", "/usr/bin/curl -fsSL https://sh.rustup.rs | sh -s -- -y -c rls"]

# Give vscode user permissions to /usr/local
RUN ["/usr/bin/sudo", "/bin/chown", "-R", "root:vscode", "/usr/local"]
RUN ["/usr/bin/sudo", "/bin/chmod", "-R", "775", "/usr/local"]

# Install & enable corepack (for Yarn)
RUN ["/usr/bin/npm", "install", "--global", "corepack"]
RUN ["/usr/local/bin/corepack", "enable"]

# Install some other CLIs
RUN ["/usr/bin/npm", "install", "--global", "nx"]
RUN ["/usr/bin/npm", "install", "--global", "prettier"]
RUN ["/usr/bin/npm", "install", "--global", "typescript"]
