default:
    @just --list

test-core *args:
    #!/usr/bin/env bash
    export DATABASE_URL="${DATABASE_URL:-postgres://packrat:packrat@localhost:5432/packrat?sslmode=disable}"
    cargo test --workspace --exclude packrat_ui {{args}}

test-ui *args:
    #!/usr/bin/env bash
    set -euo pipefail
    just build-ui-css
    cargo test --manifest-path crates/packrat_ui/Cargo.toml --workspace {{args}}

# HTTP API (Axum). Needs Postgres (e.g. `docker compose up -d postgres`).
run-api:
    #!/usr/bin/env bash
    set -euo pipefail
    export DATABASE_URL="${DATABASE_URL:-postgres://packrat:packrat@localhost:5432/packrat?sslmode=disable}"
    export LISTEN_ADDR="${LISTEN_ADDR:-127.0.0.1:3000}"
    export SQLX_OFFLINE="${SQLX_OFFLINE:-true}"
    cargo run -p packrat_api

# Regenerate `packages/ui/assets/tailwind.css` from RSX `@source` paths.
# Run after adding/changing Tailwind classes (or once before `just serve-web`).
# Installs npm deps in `crates/packrat_ui/packages/ui` on first use.
build-ui-css:
    #!/usr/bin/env bash
    set -euo pipefail
    cd "{{ justfile_directory() }}/crates/packrat_ui/packages/ui"
    if [[ ! -d node_modules ]]; then npm install; fi
    npm run build

# Packrat web UI (Dioxus). From repo root; needs `dx` on PATH.
# Starts Tailwind `npm run watch` in the background (regenerates `packages/ui/assets/tailwind.css`
# when RSX changes), then `dx serve` in `packages/web`. Ctrl+C stops both.
# Set `PACKRAT_WEB_NO_TAILWIND=1` to skip the watcher if you already run it elsewhere.
serve-web *args:
    #!/usr/bin/env bash
    set -euo pipefail
    root="{{ justfile_directory() }}"
    ui="$root/crates/packrat_ui/packages/ui"
    web="$root/crates/packrat_ui/packages/web"
    cd "$ui"
    if [[ ! -d node_modules ]]; then npm install; fi
    npm run build
    if [[ "${PACKRAT_WEB_NO_TAILWIND:-}" != "1" ]]; then
        npm run watch &
        tw_pid=$!
        cleanup() {
            kill "$tw_pid" 2>/dev/null || true
            wait "$tw_pid" 2>/dev/null || true
        }
        trap cleanup EXIT INT TERM
    fi
    cd "$web"
    dx serve {{args}}

coverage:
    #!/usr/bin/env bash
    export DATABASE_URL="${DATABASE_URL:-postgres://packrat:packrat@localhost:5432/packrat?sslmode=disable}"
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

emulator-create:
    avdmanager create avd --name phone --package 'system-images;android-34;google_apis;x86_64'

emulator-run:
    emulator -avd phone -skin 720x1280 -noaudio -no-snapshot-load -no-snapshot
