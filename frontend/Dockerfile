# syntax=docker/dockerfile:1.2
FROM rust:1.74-bookworm AS builder

WORKDIR /usr/src/cochrane

RUN rustup target add wasm32-unknown-unknown \
    && cargo install dioxus-cli --version 0.4.1
COPY . .
RUN --mount=type=cache,target=/usr/local/cargo/git \
    --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,sharing=private,target=/usr/src/cochrane/target \
    cd frontend \
    && dx build --features web --release \
    && cargo build --features ssr --release \
    && cp ../target/release/frontend ../cochrane-frontend


FROM debian:bookworm

RUN apt-get update && apt-get install -y libssl3 && apt clean && rm -rf /var/lib/apt/lists/*

COPY --from=builder /usr/src/cochrane/cochrane-frontend /usr/local/bin/cochrane-frontend
COPY --from=builder /usr/src/cochrane/frontend/dist /usr/share/web/dist
COPY frontend/docker-entrypoint.sh /usr/local/bin/docker-entrypoint.sh
RUN chmod +x /usr/local/bin/docker-entrypoint.sh

WORKDIR /usr/share/web
EXPOSE 8080
ENTRYPOINT [ "docker-entrypoint.sh" ]
CMD [ "cochrane-frontend" ]
