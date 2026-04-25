default:
    @just --list

test:
    cargo test --workspace

coverage:
    cargo tarpaulin --workspace --timeout 300 --out Html --output-dir coverage

# Regenerate `.sqlx/` after changing `query_*!` SQL or migrations.
# Requires `DATABASE_URL` and a database with migrations applied (e.g. `docker compose up -d postgres`).
sqlx-prepare:
    #!/usr/bin/env bash
    set -euo pipefail
    : "${DATABASE_URL:?set DATABASE_URL (e.g. postgres://packrat:packrat@127.0.0.1:5432/packrat?sslmode=disable)}"
    sqlx migrate run --source crates/packrat_infrastructure/migrations
    cargo sqlx prepare --workspace --database-url "$DATABASE_URL" -- --all-targets