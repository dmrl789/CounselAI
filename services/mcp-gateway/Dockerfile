# syntax=docker/dockerfile:1

FROM rust:1.81 as builder
WORKDIR /app

# Create a new empty shell project
RUN USER=root cargo new --bin mcp-gateway
WORKDIR /app/mcp-gateway

# Copy manifests
COPY services/mcp-gateway/Cargo.toml ./Cargo.toml

# Build dependencies
RUN cargo build --release || true

# Copy source and build
COPY services/mcp-gateway/src ./src
RUN cargo build --release

FROM debian:bookworm-slim
WORKDIR /app
RUN apt-get update && apt-get install -y libopenblas-dev && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/mcp-gateway/target/release/mcp-gateway /usr/local/bin/mcp-gateway
COPY services/mcp-gateway/models /models
ENV LOCAL_MODEL_PATH=/models/mistral-7b-instruct.Q4_K_M.gguf
EXPOSE 8080
CMD ["/usr/local/bin/mcp-gateway"]
