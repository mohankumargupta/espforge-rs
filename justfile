set shell := ["sh", "-c"]
set windows-shell := ["powershell", "-c"]

_main:
    @just --list

prerequisites:
    cargo install cargo-binstall
    cargo binstall espup
    cargo binstall esp-generate

update:
    espup update
    cargo binstall esp-generate

build:
    cargo build

test:
    cargo build
    cargo test

check:
    cargo check

run:
    cargo run -- compile examples/blink.toml

bare:
    cargo run -- compile examples/bare.toml

