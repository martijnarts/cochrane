FROM rust:1.73-buster AS builder

WORKDIR /usr/src/cochrane

RUN rustup target add wasm32-unknown-unknown \
    && cargo install dioxus-cli --version 0.4.1
COPY . .
RUN cd frontend \
    && dx build --features web --platform web --release

FROM caddy:2.7.5-alpine

COPY --from=builder /usr/src/cochrane/frontend/dist /usr/share/caddy