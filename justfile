CFLAGS := ""

default:
    just --list

test:
    cargo test --all-features {{CFLAGS}}
test-no-capture:
    cargo test --all-features {{CFLAGS}} -- --nocapture

build:
    cargo build {{CFLAGS}}

alias doc := docs
docs:
    cargo doc --all-features {{CFLAGS}}

docs-rs:
    RUSTDOCFLAGS="--cfg docsrs" cargo +nightly doc --no-deps --all-features {{CFLAGS}}

fmt:
    cargo fmt {{CFLAGS}}
lint:
    cargo clippy --all-targets --all-features {{CFLAGS}}
lint-fix:
    cargo clippy --all-targets --all-features --fix --allow-dirty {{CFLAGS}}
