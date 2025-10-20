# -----------------------------------------------------
# Counsel AI — MCP Gateway Dockerfile
# -----------------------------------------------------
# Acts as the privacy firewall between GPT-5 and local data.
# Handles sanitization, reasoning requests, and encryption.

FROM rust:1.81 as builder
WORKDIR /app

# Install required build dependencies
RUN apt-get update && apt-get install -y pkg-config libssl-dev curl && rm -rf /var/lib/apt/lists/*

# Copy source files
COPY . .

# Build release binary
RUN cargo build --release

# -----------------------------------------------------
# Final lightweight runtime
# -----------------------------------------------------
FROM debian:bookworm-slim
WORKDIR /app

# Install minimal runtime dependencies
RUN apt-get update && apt-get install -y ca-certificates curl && rm -rf /var/lib/apt/lists/*

# Copy binary from builder
COPY --from=builder /app/target/release/mcp-gateway /usr/local/bin/mcp-gateway

# Create data directories
RUN mkdir -p /data/encrypted && mkdir -p /logs

# Expose REST API
EXPOSE 5142

ENV RUST_LOG=info

# Health check endpoint
HEALTHCHECK CMD curl -f http://localhost:5142/health || exit 1

# Run the gateway
CMD ["mcp-gateway"]
