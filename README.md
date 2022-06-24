# Devcon

Use [devcontainers](https://code.visualstudio.com/docs/remote/containers) outside of Visual Studio Code.

## Notice

`devcon` is currently beta software. It is not ready for serious use.

## Installation

Download the binary from the [latest release](https://github.com/guitsaru/devcon/releases).

## Usage

* `devcon` - starts the container specified in `.devcontainer/devcontainer.yml`
* `devcon rebuild [--no-cache]` - rebuilds and starts the container (optionally without cache). Run this if you make changes to the Dockerfile.

## SSH Agent

`devcon` will automatically give the container access to your ssh agent. This will allow you to use your ssh keys for ssh or git without needing to copy them in.

## Configuration

Create a file in `~/.config/devcon/config.toml`

```toml
# Can be either "podman" or "docker", defaults to "docker"
provider = "docker"

# The list of dotfiles you want to copy into the container. Files are relative to your how directory.
# These can be files or directories.
dotfiles = [
	".zshrc",
	".config/nvim",
]
```

## Supported Container Engines

- [x] docker
- [x] podman
- [x] docker-compose
- [ ] podman-compose
