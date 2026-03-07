# 🏗️ Gateway Stage: Build Frontend
FROM rust:1.88 AS frontend-builder

RUN apt-get update && apt-get install -y \
    pkg-config \
    curl \
    git \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

RUN rustup target add wasm32-unknown-unknown
RUN curl -L https://github.com/trunk-rs/trunk/releases/download/v0.21.14/trunk-x86_64-unknown-linux-gnu.tar.gz | tar -xzf - -C /usr/local/bin

WORKDIR /app
COPY frontend/Cargo.toml frontend/Cargo.lock* ./
COPY frontend/Trunk.toml ./
COPY frontend/src ./src
COPY frontend/index.html ./

RUN trunk build --release

# 🚀 Final Stage: Nginx
FROM nginx:alpine

# Copy built files
COPY --from=frontend-builder /app/dist /usr/share/nginx/html

# Use Nginx template for dynamic BACKEND_URL substitution
COPY nginx/default.conf.template /etc/nginx/templates/default.conf.template

# Default backend URL (can be overridden by Render env vars)
ENV BACKEND_URL=http://gravity-backend:8080

EXPOSE 80
CMD ["nginx", "-g", "daemon off;"]
