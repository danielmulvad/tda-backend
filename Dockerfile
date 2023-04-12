# Builder
FROM rust:1.68-slim as builder

WORKDIR /usr/src/tda-server

COPY ./Cargo.toml ./Cargo.toml
COPY ./Cargo.lock ./Cargo.lock
COPY ./src/ ./src/
COPY ./.env ./.env

RUN apt update && apt install pkg-config openssl libssl-dev ca-certificates -y
RUN cargo install --path . --target-dir ./target

# Runner
FROM debian:buster-slim

COPY --from=builder /usr/src/tda-server /usr/local/bin/tda-server

WORKDIR /usr/local/bin/tda-server/target/release

COPY .env ./.env

RUN --mount=type=secret,id=TDA_API_KEY awk '{print "\nTDA_API_KEY="$1}' /run/secrets/TDA_API_KEY >> .env
RUN apt update && apt install pkg-config openssl libssl-dev ca-certificates -y

EXPOSE 3000

ENTRYPOINT ["./tda-server"]
