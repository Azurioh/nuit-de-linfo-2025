# Base stage
FROM rust:1.83-slim-bookworm AS chef
WORKDIR /usr/src/app
RUN apt-get update && apt-get install -y pkg-config libssl-dev && rm -rf /var/lib/apt/lists/*
RUN cargo install --locked cargo-chef

# Planner stage
FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

# Builder stage
FROM chef AS builder
COPY --from=planner /usr/src/app/recipe.json recipe.json
# Build dependencies - this is the caching layer!
RUN cargo chef cook --release --recipe-path recipe.json

# Build application
COPY . .
ENV RUSTFLAGS="-C target-cpu=native"
RUN cargo build --release --bin backend

# Runtime stage
FROM debian:bookworm-slim

WORKDIR /usr/local/bin

# Install runtime dependencies
RUN apt-get update && apt-get install -y ca-certificates libssl3 openssl && rm -rf /var/lib/apt/lists/*

# Copy the binary from the builder stage
COPY --from=builder /usr/src/app/target/release/backend .
COPY entrypoint.sh .

# Expose the port
EXPOSE 8080

# Run the binary via entrypoint
ENTRYPOINT ["./entrypoint.sh"]
CMD ["./backend"]
