FROM rust:1.73-buster AS builder

WORKDIR /usr/src/cochrane

COPY . .
RUN cargo build --release -p backend

ENTRYPOINT ["/usr/src/cochrane/target/release/backend"]