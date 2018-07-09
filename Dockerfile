FROM rust:1.27-stretch

WORKDIR /app

COPY src src
COPY Cargo.toml .

RUN cargo build

ENV PARTNER_ADDRESS=''
ENV RUST_LOG=info

ENTRYPOINT cargo run
