# --- Build ---
FROM rust:latest AS builder
LABEL stage=builder
WORKDIR /app

# Install needed dependencies for sqlx
RUN apt-get update && apt-get install -y \
    pkg-config \
    libpq-dev \
    && rm -rf /var/lib/apt/lists/*

# Cache dependencies first (faster rebuilds)
COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release
RUN rm -rf src

# Build the app
COPY src ./src
COPY .sqlx ./.sqlx
COPY migrations ./migrations
ENV SQLX_OFFLINE=true
RUN touch src/main.rs && cargo build --release

# --- Runtime ---
FROM debian:stable-slim AS runtime
#FROM gcr.io/distroless/cc AS runtime

WORKDIR /app

COPY --from=builder /app/target/release/rustweb ./server
COPY migrations ./migrations

EXPOSE 8000

CMD ["./server"]