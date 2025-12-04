# Builder stage
FROM rust:1.83-slim-bookworm as builder

WORKDIR /usr/src/app

# Install build dependencies
RUN apt-get update && apt-get install -y pkg-config libssl-dev && rm -rf /var/lib/apt/lists/*

# Copy manifests
COPY Cargo.toml Cargo.lock ./

# Create dummy main to cache dependencies
# This is a common pattern to cache cargo build dependencies
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release

# Remove dummy build artifacts
RUN rm -f target/release/deps/backend*

# Copy source code
COPY . .

# Build the actual application
RUN cargo build --release

# Runtime stage
FROM debian:bookworm-slim

WORKDIR /usr/local/bin

# Install runtime dependencies
RUN apt-get update && apt-get install -y ca-certificates libssl3 && rm -rf /var/lib/apt/lists/*

# Copy the binary from the builder stage
COPY --from=builder /usr/src/app/target/release/backend .

# Expose the port
EXPOSE 8080

# Run the binary
CMD ["./backend"]
