
ARG RUST_VERSION=1

FROM node:22-bookworm AS tailwind

WORKDIR /repo/crates/packrat_ui/packages/ui

COPY crates/packrat_ui/packages/ui/package.json crates/packrat_ui/packages/ui/package-lock.json ./
COPY crates/packrat_ui/packages/ui/input.css ./
COPY crates/packrat_ui/packages/ui/src ./src
COPY crates/packrat_ui/packages/web ../web
COPY crates/packrat_ui/packages/desktop ../desktop
COPY crates/packrat_ui/packages/mobile ../mobile

RUN npm ci && npm run build

FROM rust:${RUST_VERSION}-bookworm AS ui-builder

ARG DIOXUS_CLI_VERSION=0.7.1

RUN apt-get update && apt-get install -y --no-install-recommends \
    build-essential \
    ca-certificates \
    curl \
    git \
    libssl-dev \
    pkg-config \
    && rm -rf /var/lib/apt/lists/*

RUN rustup target add wasm32-unknown-unknown

ENV CARGO_HOME=/usr/local/cargo
RUN cargo install dioxus-cli --version "${DIOXUS_CLI_VERSION}" --locked

WORKDIR /repo/crates/packrat_ui

COPY crates/packrat_ui/ ./

RUN mkdir -p packages/ui/assets
COPY --from=tailwind /repo/crates/packrat_ui/packages/ui/assets/tailwind.css ./packages/ui/assets/tailwind.css

ENV CARGO_TARGET_DIR=/repo/crates/packrat_ui/target

RUN dx build --release -p web

FROM rust:${RUST_VERSION}-bookworm AS api-builder

WORKDIR /app

RUN apt-get update && apt-get install -y --no-install-recommends libssl-dev pkg-config \
    && rm -rf /var/lib/apt/lists/*

COPY Cargo.toml Cargo.lock ./
COPY .sqlx ./.sqlx
COPY crates ./crates

ENV SQLX_OFFLINE=true
RUN cargo build --release -p packrat_api

FROM debian:bookworm-slim AS runtime

RUN apt-get update \
    && apt-get install -y --no-install-recommends ca-certificates \
    && rm -rf /var/lib/apt/lists/* \
    && useradd --system --no-create-home --uid 65532 packrat

COPY --from=api-builder /app/target/release/packrat_api /usr/local/bin/packrat_api
COPY --from=ui-builder --chown=65532:65532 /repo/crates/packrat_ui/target/dx/web/release/web/public /srv/packrat/ui

ENV LISTEN_ADDR=0.0.0.0:3000
ENV PACKRAT_STATIC_UI=/srv/packrat/ui

USER 65532:65532

EXPOSE 3000

ENTRYPOINT ["/usr/local/bin/packrat_api"]
