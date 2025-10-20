FROM rust:1.77 AS builder
WORKDIR /app

COPY Cargo.toml ./
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release

COPY src ./src
RUN cargo build --release

FROM debian:bookworm-slim
WORKDIR /app

RUN apt-get update && apt-get install -y --no-install-recommends ca-certificates && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/mcp-gateway /usr/local/bin/mcp-gateway

ENV RUST_LOG=info
EXPOSE 5142

CMD ["/usr/local/bin/mcp-gateway"]
