FROM fedora:latest

ARG USER=devcon
ENV SHELL=/usr/bin/zsh

RUN dnf update -y
RUN dnf install -y procps zsh neovim clang fd-find ripgrep fzf exa bat git-delta tmux python3-neovim starship gh lldb

# RUN useradd -ms /usr/bin/zsh $USER && \
#     usermod -aG wheel $USER && \
#     echo "${USER}        ALL=(ALL)       NOPASSWD: ALL" >> /etc/sudoers

RUN dnf install -y rust cargo clippy rustfmt rust-src

# USER $USER

