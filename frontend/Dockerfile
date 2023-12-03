FROM rust:1.74-bookworm AS builder

WORKDIR /usr/src/cochrane

RUN rustup target add wasm32-unknown-unknown \
    && cargo install dioxus-cli --version 0.4.1
COPY . .
RUN cd frontend \
    && dx build --features web --release \
    && cargo build --features ssr --release

FROM debian:bookworm

COPY --from=builder /usr/src/cochrane/target/release/frontend /usr/local/bin/frontend
COPY --from=builder /usr/src/cochrane/frontend/dist /usr/share/web/dist

WORKDIR /usr/share/web
EXPOSE 8080
ENTRYPOINT [ "frontend" ]
