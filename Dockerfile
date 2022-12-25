# Build client assets (JavaScript and CSS)
FROM node:19-alpine as build-node
WORKDIR /usr/src/app
COPY ./esbuild.js ./
COPY ./package*.json ./
RUN npm install
RUN npm install --unsafe-perm -g sass
COPY ./frontend/scripts ./frontend/scripts
COPY ./frontend/styles ./frontend/styles
COPY static static
WORKDIR /usr/src/app
RUN npm run build

FROM lukemathwalker/cargo-chef:latest-rust-1.66.0 as chef
WORKDIR /app
RUN apt update && apt install lld clang -y

FROM chef as planner
COPY . .
# Compute a lock-like file for our project
RUN cargo chef prepare  --recipe-path recipe.json

FROM chef as builder
COPY --from=planner /app/recipe.json recipe.json
# Build our project dependencies, not our application!
RUN cargo chef cook --release --recipe-path recipe.json
COPY . .
ENV SQLX_OFFLINE true
# Build our project
RUN cargo build --release --bin opusclassical

FROM debian:bullseye-slim AS runtime
WORKDIR /app
RUN apt-get update -y \
    && apt-get install -y --no-install-recommends openssl ca-certificates \
    # Clean up
    && apt-get autoremove -y \
    && apt-get clean -y \
    && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/opusclassical opusclassical
COPY --from=build-node /usr/src/app/static static
COPY templates templates
EXPOSE 8000
ENTRYPOINT ["./opusclassical"]