default:
    just --list

dev:
    RUST_LOG=trace bacon clippy

run *args:
    RUST_LOG=trace cargo run -- {{args}}

run-release *args:
    cargo run --release -- {{args}}

clean:
    cargo clean

test:
    cargo test

alias d := dev
alias r := run
alias rr := run-release
alias c := clean
alias t := test
