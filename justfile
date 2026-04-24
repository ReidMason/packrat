default:
    @just --list

coverage:
    cargo tarpaulin --workspace --timeout 300 --out Html --output-dir coverage