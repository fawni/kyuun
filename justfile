set windows-shell := ["pwsh.exe", "-NoLogo", "-Command"]

_default:
    @just --list

# Install to PATH
install:
    cargo install --path .

# Uninstall
uninstall:
    cargo uninstall kyuun

# Exessive clippy lints
lint:
    cargo clippy --locked -- -W clippy::pedantic -W clippy::nursery