default:
    just --list

dev:
    RUST_LOG=trace cargo watch -q -c -x "run -- README.md Cargo.lock"

check:
    bacon clippy

run *args:
    RUST_LOG=trace cargo run -- {{args}}

run-release *args:
    cargo run --release -- {{args}}

clean:
    cargo clean

test:
    cargo test

install:
    cargo install --path .

alias d := dev
alias ch := check
alias r := run
alias rr := run-release
alias c := clean
alias t := test
alias i := install
