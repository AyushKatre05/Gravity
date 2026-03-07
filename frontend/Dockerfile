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

# Copy built files to volume mount point
FROM alpine:latest

COPY --from=builder /app/dist /dist

VOLUME ["/output"]

ENTRYPOINT sh -c "cp -r /dist/* /output/ && echo 'Frontend build copied to volume'"
