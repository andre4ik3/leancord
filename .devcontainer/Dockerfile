FROM mcr.microsoft.com/vscode/devcontainers/base:bullseye

RUN ["/usr/bin/apt", "update", "-y"]
RUN ["/usr/bin/apt", "upgrade", "-y"]

ENV DENO_INSTALL="/usr/local"

RUN ["/usr/bin/apt", "install", "-y", "nodejs"]
RUN ["sh", "-c", "/usr/bin/curl -fsSL https://deno.land/install.sh | sh"]
RUN ["sh", "-c", "/usr/bin/curl -fsSL https://sh.rustup.rs | sh -s -- -y -c rls"]
