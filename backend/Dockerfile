# ─── Stage 1: Builder ────────────────────────────────────────────────────────
FROM rust:1.88-slim AS builder

# Install build essentials
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    git \
    ca-certificates \
    build-essential \
    gcc \
    g++ \
    make \
    && rm -rf /var/lib/apt/lists/*

# ⚠️ NOTE: This Dockerfile must be built from the project ROOT context
# Usage: docker build -t gravity-backend -f backend/Dockerfile .

WORKDIR /app

# Copy manifest files
COPY backend/Cargo.toml ./

# Copy src, migrations and build script
COPY backend/src ./src
COPY backend/migrations ./migrations
COPY backend/build.rs* ./

# Build release binary
RUN cargo build --release

# ─── Stage 2: Runtime ────────────────────────────────────────────────────────
FROM debian:bookworm-slim AS runtime

RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    curl \
    git \
    libgcc-s1 \
    libstdc++6 \
    libtree-sitter0 \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy binary from builder
COPY --from=builder /app/target/release/gravity-backend /usr/local/bin/gravity-backend

# Copy migrations so sqlx::migrate!() finds them at runtime
COPY --from=builder /app/migrations ./migrations

# Create the /analyze mount point
RUN mkdir -p /analyze

# Healthcheck
HEALTHCHECK --interval=30s --timeout=10s --start-period=40s --retries=3 \
    CMD curl -f http://localhost:8080/health || exit 1

# Render and other platforms inject PORT; default to 8080
ENV PORT=8080
ENV ANALYZE_PATH=/analyze
EXPOSE 8080

CMD ["gravity-backend"]
