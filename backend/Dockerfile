# ─── Stage 1: Builder ────────────────────────────────────────────────────────
FROM rust:1.88-slim AS builder

# Install essential build tooling
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    git \
    ca-certificates \
    build-essential \
    gcc \
    g++ \
    make \
    libtree-sitter-dev \
    libtree-sitter0 \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Build tree-sitter-rust grammar library
RUN cd /tmp && \
    git clone https://github.com/tree-sitter/tree-sitter-rust.git && \
    cd tree-sitter-rust && \
    gcc -fPIC -shared src/parser.c src/scanner.c -o libtree_sitter_rust.so && \
    cp libtree_sitter_rust.so /usr/local/lib/ && \
    ldconfig

# Set library paths for pkg-config
ENV PKG_CONFIG_PATH="/usr/lib/x86_64-linux-gnu/pkgconfig:/usr/lib/pkgconfig:/usr/local/lib/pkgconfig"
ENV LD_LIBRARY_PATH="/usr/lib/x86_64-linux-gnu:/usr/local/lib"

# Copy manifest files
COPY Cargo.toml ./

# Copy src and migrations
COPY src ./src
COPY migrations ./migrations
COPY build.rs ./

# Build release binary
RUN cargo build --release

# ─── Stage 2: Runtime ────────────────────────────────────────────────────────
FROM debian:bookworm-slim AS runtime

RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    curl \
    git \
    gcc \
    libgcc-s1 \
    libstdc++6 \
    libtree-sitter0 \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy binary from builder
COPY --from=builder /app/target/release/gravity-backend /usr/local/bin/gravity-backend

# Copy tree-sitter-rust library from builder
COPY --from=builder /usr/local/lib/libtree_sitter_rust.so /usr/local/lib/

# Register the library
RUN ldconfig

# Set library path
ENV LD_LIBRARY_PATH="/usr/local/lib:/usr/lib/x86_64-linux-gnu"

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

EXPOSE ${PORT}

HEALTHCHECK --interval=30s --timeout=10s --start-period=30s --retries=3 \
    CMD curl -f http://localhost:${PORT}/health || exit 1

CMD ["gravity-backend"]
