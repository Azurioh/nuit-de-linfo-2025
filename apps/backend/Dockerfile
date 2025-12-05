FROM rust:1.83-slim-bookworm AS chef
WORKDIR /app
RUN cargo install --locked cargo-chef

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
RUN apt-get update && apt-get install -y pkg-config libssl-dev && rm -rf /var/lib/apt/lists/*
RUN cargo chef cook --release --recipe-path recipe.json
COPY . .
ENV RUSTFLAGS="-C link-arg=-s"
RUN cargo build --release --bin backend

FROM gcr.io/distroless/cc-debian12
WORKDIR /app
COPY --from=builder /app/target/release/backend .

EXPOSE 8080
CMD ["./backend"]
