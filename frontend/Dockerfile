FROM rust:1.88 AS builder

RUN apt-get update && apt-get install -y \
    pkg-config \
    curl \
    git \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

RUN rustup target add wasm32-unknown-unknown

RUN curl -L https://github.com/trunk-rs/trunk/releases/download/v0.21.14/trunk-x86_64-unknown-linux-gnu.tar.gz | tar -xzf - -C /usr/local/bin

WORKDIR /app

COPY Cargo.toml Cargo.lock* ./
COPY Trunk.toml rust-toolchain.toml* ./
COPY src ./src
COPY index.html ./

RUN trunk build --release

FROM nginx:alpine AS static

RUN apk add --no-cache tini

COPY --from=builder /app/dist /usr/share/nginx/html

EXPOSE 80

HEALTHCHECK --interval=30s --timeout=10s --start-period=10s --retries=3 \
    CMD wget --no-verbose --tries=1 --spider http://localhost/index.html || exit 1

ENTRYPOINT ["/sbin/tini", "--"]
CMD ["nginx", "-g", "daemon off;"]
