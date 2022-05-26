ARG IMAGE
FROM ${IMAGE}

ARG USERNAME="vscode"
ENV DENO_DIR="/usr/local/deno"
ENV DENO_INSTALL="/usr/local"
ENV RUSTUP_HOME="/usr/local"
ENV CARGO_HOME="/usr/local"

USER root

# Make sure repositories are up to date
RUN ["/usr/bin/apt", "update", "-y"]
RUN ["/usr/bin/apt", "upgrade", "-y"]

# Give vscode user permissions to /usr/local
RUN "/bin/mkdir /usr/local/deno"
RUN "/bin/chown -R root:${USERNAME} /usr/local /usr/local/bin /usr/local/deno"
RUN "/bin/chmod -R 775 /usr/local /usr/local/bin /usr/local/deno"

# Install base dependencies.
RUN ["/usr/bin/apt", "install", "-y", "build-essential"]
RUN ["/usr/bin/apt", "install", "-y", "libssl-dev"]
RUN ["/usr/bin/apt", "install", "-y", "pkg-config"]
RUN ["/bin/bash", "-c", "curl -fsSL https://deb.nodesource.com/setup_lts.x | bash -"]

# Setup Node.JS
RUN ["/usr/bin/apt", "install", "-y", "nodejs"]
RUN ["/usr/bin/corepack", "enable"]
RUN ["/usr/bin/npm", "install", "--global", "nx"]
RUN ["/usr/bin/npm", "install", "--global", "prettier"]
RUN ["/usr/bin/npm", "install", "--global", "typescript"]

# Install some other things under the vscode user.
USER ${USERNAME}
RUN ["sh", "-c", "/usr/bin/curl -fsSL https://sh.rustup.rs | sh -s -- --no-modify-path -y -t wasm32-unknown-unknown -c rls"]
RUN ["/usr/local/bin/cargo", "install", "--locked", "wasm-bindgen-cli"]
RUN ["/usr/local/bin/cargo", "install", "--locked", "wasm-pack"]
RUN ["sh", "-c", "/usr/bin/curl -fsSL https://deno.land/install.sh | sh"]
