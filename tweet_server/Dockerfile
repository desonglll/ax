# Stage 1: Build the Rust binary
FROM rust:1.78-slim AS builder

RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy the Cargo workspace configurations and the crates
COPY Cargo.toml Cargo.lock ./
COPY ai ./ai
COPY tweet_server ./tweet_server

# Build the release binary
WORKDIR /app/tweet_server
ARG DATABASE_URL
ENV DATABASE_URL=${DATABASE_URL}
RUN cargo build --release

# Stage 2: Minimal runtime container
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy the compiled binary from the builder stage
COPY --from=builder /app/target/release/tweet_server /app/tweet_server

EXPOSE 8000
CMD ["/app/tweet_server"]
