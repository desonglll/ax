# Build from the repository root: docker build -f tweet_server/Dockerfile .
FROM rust:1-slim-bookworm AS builder
RUN apt-get update && apt-get install -y --no-install-recommends pkg-config libssl-dev \
    && rm -rf /var/lib/apt/lists/*
WORKDIR /app
COPY Cargo.toml Cargo.lock ./
COPY .sqlx ./.sqlx
COPY migrations ./migrations
COPY tweet_server ./tweet_server
# Queries are checked against the committed .sqlx cache; no database at build time.
ENV SQLX_OFFLINE=true
RUN cargo build --release --manifest-path tweet_server/Cargo.toml

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y --no-install-recommends ca-certificates libssl3 \
    && rm -rf /var/lib/apt/lists/*
WORKDIR /app
COPY --from=builder /app/target/release/tweet_server /app/tweet_server
ENV UPLOAD_DIR=/data/uploads PORT=8000
VOLUME /data/uploads
EXPOSE 8000
CMD ["/app/tweet_server"]
