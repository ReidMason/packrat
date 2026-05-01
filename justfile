# UI workspace commands
mod ui "./crates/packrat_ui/packages/ui/justfile"

db_url := "postgres://packrat:packrat@localhost:5432/packrat?sslmode=disable"

default:
    @just --list

# Run tests from the packrat CORE workspace
[group('tests')]
test-core *args:
    #!/usr/bin/env bash
    export DATABASE_URL="{{db_url}}"
    cargo test --workspace --exclude packrat_ui {{args}}

# Run tests from the packrat UI workspace
[group('tests')]
test-ui *args:
    #!/usr/bin/env bash
    set -euo pipefail
    just ui tailwind-build
    cargo test --manifest-path crates/packrat_ui/Cargo.toml --workspace {{args}}

# Generate an HTML code coverage report using tarpaulin
[group('tests')]
coverage:
    #!/usr/bin/env bash
    export DATABASE_URL="{{db_url}}"
    cargo tarpaulin --workspace --timeout 300 --out Html --output-dir coverage


# HTTP API (Axum). Needs Postgres (e.g. `docker compose up -d postgres`).
[group('api')]
run-api:
    #!/usr/bin/env bash
    set -euo pipefail

    if lsof -t -i :3000 > /dev/null; then
        echo "Cleaning up existing API process on port 3000..."
        kill -9 $(lsof -t -i :3000) || true
        sleep 0.5
    fi

    export DATABASE_URL="{{db_url}}"
    export LISTEN_ADDR="${LISTEN_ADDR:-127.0.0.1:3000}"
    export SQLX_OFFLINE="${SQLX_OFFLINE:-true}"
    cargo run -p packrat_api

# Run both the API and the UI - args are [web, desktop, android]
[group('fullstack-dev')]
dev platform="web":
    #!/usr/bin/env bash
    set -euo pipefail

    echo "Starting API..."
    just run-api &
    api_pid=$!

    cleanup() {
        echo "Shutting down stack..."
        kill "$api_pid" 2>/dev/null || true
    }
    trap cleanup EXIT INT TERM

    echo "Starting UI for {{platform}}..."
    just ui serve-{{platform}}

# Regenerate `.sqlx/` after changing `query_*!` SQL or migrations.
# Needs Postgres up (e.g. `docker compose up -d postgres`)
[group('database')]
sqlx-prepare:
    #!/usr/bin/env bash
    set -euo pipefail
    export PATH="${HOME}/.cargo/bin:${PATH}"
    export DATABASE_URL="{{db_url}}"
    cargo sqlx migrate run --source crates/packrat_infrastructure/migrations
    cargo sqlx prepare --workspace --database-url "$DATABASE_URL" -- --all-targets

# Run this once to build a 'phone'
[group('mobile')]
emulator-create:
    avdmanager create avd --name phone --package 'system-images;android-34;google_apis;x86_64'

# dx serve mobile seems to run this for you so not need most of the time
[group('mobile')]
emulator-run:
    emulator -avd phone -skin 720x1280 -noaudio -no-snapshot-load -no-snapshot
