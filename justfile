default:
    @just --list

test:
    cargo test --workspace

coverage:
    cargo tarpaulin --workspace --timeout 300 --out Html --output-dir coverage

# Regenerate `.sqlx/` after changing `query_*!` SQL or migrations.
# Needs Postgres up (e.g. `docker compose up -d postgres`)
sqlx-prepare:
    #!/usr/bin/env bash
    set -euo pipefail
    export PATH="${HOME}/.cargo/bin:${PATH}"
    export DATABASE_URL="${DATABASE_URL:-postgres://packrat:packrat@localhost:5432/packrat?sslmode=disable}"
    cargo sqlx migrate run --source crates/packrat_infrastructure/migrations
    cargo sqlx prepare --workspace --database-url "$DATABASE_URL" -- --all-targets