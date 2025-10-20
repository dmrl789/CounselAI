# -----------------------------------------------------
# Counsel AI — MCP Gateway Dockerfile
# -----------------------------------------------------
# Acts as the privacy firewall between GPT-5 and local data.
# Handles sanitization, reasoning requests, encryption, and local LLM reasoning (llama.cpp).

FROM rust:1.81 as builder
WORKDIR /app

# Install required build dependencies
RUN apt-get update && apt-get install -y pkg-config libssl-dev libopenblas-dev curl && rm -rf /var/lib/apt/lists/*

# Copy manifest files
COPY Cargo.toml Cargo.lock ./
COPY src ./src

# Build release binary
RUN cargo build --release

# -----------------------------------------------------
# Final lightweight runtime
# -----------------------------------------------------
FROM debian:bookworm-slim
WORKDIR /app

# Install minimal runtime dependencies
RUN apt-get update && apt-get install -y --no-install-recommends ca-certificates curl libopenblas-dev && rm -rf /var/lib/apt/lists/*

# Copy compiled binary
COPY --from=builder /app/target/release/mcp-gateway /usr/local/bin/mcp-gateway

# Copy models (if bundled)
COPY services/mcp-gateway/models /models

# Create runtime directories
RUN mkdir -p /data/encrypted /logs

# Environment
ENV RUST_LOG=info
ENV LOCAL_MODEL_PATH=/models/mistral-7b-instruct.Q4_K_M.gguf

# Expose REST API
EXPOSE 5142

# Health check for Docker
HEALTHCHECK CMD curl -f http://localhost:5142/health || exit 1

# Default command
CMD ["/usr/local/bin/mcp-gateway"]
