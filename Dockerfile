# Builder
FROM rust:1.68-slim as builder

WORKDIR /usr/src/tda-server

COPY ./Cargo.toml ./Cargo.toml
COPY ./Cargo.lock ./Cargo.lock
COPY ./src/ ./src/
COPY ./.env ./.env

RUN apt update && apt install pkg-config openssl -y
RUN cargo install --path . --target-dir ./target

# Runner
FROM debian:buster-slim

COPY --from=builder /usr/src/tda-server /usr/local/bin/tda-server
WORKDIR /usr/local/bin/tda-server/target/release

COPY .env ./.env

EXPOSE 3000

ENTRYPOINT ["./tda-server"]
