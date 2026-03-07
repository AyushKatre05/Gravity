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

COPY --from=frontend-builder /app/dist /usr/share/nginx/html
COPY nginx/nginx.conf /etc/nginx/nginx.conf

# Replace backend host in nginx.conf if needed via entrypoint
# But here we use the name from render.yaml 'gravity-backend'
RUN sed -i 's/http:\/\/backend:8080/http:\/\/gravity-backend:8080/g' /etc/nginx/nginx.conf

EXPOSE 80
CMD ["nginx", "-g", "daemon off;"]
