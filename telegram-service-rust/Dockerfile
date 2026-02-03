FROM rust:1.88-bullseye AS builder

WORKDIR /app
COPY Cargo.toml Cargo.lock ./
COPY src ./src

RUN cargo build --release

FROM debian:bullseye-slim

RUN apt-get update \
    && apt-get install -y --no-install-recommends ca-certificates libssl1.1 \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app
COPY --from=builder /app/target/release/tg_bot_vsu /app/tg_bot_vsu

ENV RUST_LOG=info

CMD ["/app/tg_bot_vsu"]
