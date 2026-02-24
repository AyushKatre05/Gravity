FROM rust:1.85 AS builder

RUN apt-get update && apt-get install -y \
    pkg-config \
    curl \
    git \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

RUN rustup target add wasm32-unknown-unknown

RUN curl -L https://github.com/trunk-rs/trunk/releases/download/v0.21.14/trunk-x86_64-unknown-linux-gnu.tar.gz | tar -xzf - -C /usr/local/bin

WORKDIR /app

COPY . .

RUN mkdir -p dist target

RUN rm -f Cargo.lock

RUN trunk build --release

FROM nginx:alpine AS static

COPY --from=builder /app/dist /usr/share/nginx/html

EXPOSE 80

CMD ["nginx", "-g", "daemon off;"]
