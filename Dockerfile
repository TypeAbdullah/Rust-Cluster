# ── Stage 1: Build ──────────────────────────────────────
FROM rust:latest AS builder

WORKDIR /app
COPY Cargo.toml Cargo.lock* ./
COPY src/ src/
COPY frontend/ frontend/

# Build release binary
RUN cargo build --release

# ── Stage 2: Runtime ────────────────────────────────────
FROM debian:bookworm-slim

# Install runtime dependencies (git + common runtimes)
RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates \
    git \
    curl \
    && rm -rf /var/lib/apt/lists/*

# Install Node.js 20 LTS
RUN curl -fsSL https://deb.nodesource.com/setup_20.x | bash - \
    && apt-get install -y nodejs \
    && rm -rf /var/lib/apt/lists/*

# Install Python 3
RUN apt-get update && apt-get install -y --no-install-recommends \
    python3 python3-pip python3-venv \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy binary from builder
COPY --from=builder /app/target/release/rustcluster .

# Create apps directory
RUN mkdir -p apps

# Default environment
ENV PORT=8080
ENV APPS_DIR=./apps
ENV RUST_LOG=rustcluster=info

EXPOSE 8080

CMD ["./rustcluster"]
