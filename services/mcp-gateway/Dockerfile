# -----------------------------------------------------
# Counsel AI — MCP Gateway Dockerfile
# -----------------------------------------------------
# Acts as the privacy firewall between GPT-5 and local data.
# Handles sanitization, reasoning requests, and encryption.

FROM rust:1.81 as builder
WORKDIR /app

# Install required build dependencies
RUN apt-get update && apt-get install -y pkg-config libssl-dev curl && rm -rf /var/lib/apt/lists/*

# Copy project files
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
RUN apt-get update && apt-get install -y --no-install-recommends ca-certificates curl && rm -rf /var/lib/apt/lists/*

# Copy compiled binary
COPY --from=builder /app/target/release/mcp-gateway /usr/local/bin/mcp-gateway

# Create runtime directories
RUN mkdir -p /data/encrypted /logs

# Environment
ENV RUST_LOG=info

# Expose REST API
EXPOSE 5142

# Health check for Docker
HEALTHCHECK CMD curl -f http://localhost:5142/health || exit 1

# Run the gateway
CMD ["/usr/local/bin/mcp-gateway"]
