FROM rust:1-alpine AS builder
WORKDIR /app

RUN apk add --no-cache musl-dev

COPY Cargo.toml Cargo.lock ./
COPY .sqlx ./.sqlx
COPY crates ./crates

ENV SQLX_OFFLINE=true
RUN cargo build --release -p packrat

FROM scratch AS runtime

COPY --from=builder /app/target/release/packrat /packrat
# nobody user
USER 65534:65534

ENTRYPOINT ["/packrat"]
