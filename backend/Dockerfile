# syntax=docker/dockerfile:1.2
FROM rust:1.73-bookworm AS builder

WORKDIR /usr/src/cochrane

COPY . .
RUN --mount=type=cache,target=/usr/local/cargo/git \
    --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,sharing=private,target=/usr/src/cochrane/target \
    cargo build --release -p backend \
    &&  cp target/release/backend ./cochrane-backend

FROM debian:bookworm-slim
COPY --from=builder /usr/src/cochrane/cochrane-backend /usr/local/bin/cochrane-backend
CMD ["cochrane-backend"]
