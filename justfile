default:
    @just --list

test:
    cargo test --workspace

coverage:
    cargo tarpaulin --workspace --timeout 300 --out Html --output-dir coverage