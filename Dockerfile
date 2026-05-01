FROM rust:1-alpine AS builder
WORKDIR /app

RUN apk add --no-cache musl-dev

COPY Cargo.toml Cargo.lock ./
COPY .sqlx ./.sqlx
COPY crates ./crates

ENV SQLX_OFFLINE=true
RUN cargo build --release -p packrat_api

FROM scratch AS runtime

COPY --from=builder /app/target/release/packrat_api /packrat_api
# nobody user
USER 65534:65534

ENV LISTEN_ADDR=0.0.0.0:3000
ENTRYPOINT ["/packrat_api"]
