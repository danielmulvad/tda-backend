# Builder
FROM rust:1.68-slim as builder

WORKDIR /usr/src/tda-server

RUN --mount=type=bind,target=. \
  --mount=type=secret,id=PRIVATE_CERTIFICATE_CERT \
  --mount=type=secret,id=PRIVATE_CERTIFICATE_KEY

RUN mkdir ./self_signed_certs
RUN cat /run/secrets/PRIVATE_CERTIFICATE_CERT > ./self_signed_certs/cert.pem
RUN cat /run/secrets/PRIVATE_CERTIFICATE_KEY > ./self_signed_certs/key.pem

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
RUN apt update && apt install pkg-config openssl libssl-dev ca-certificates -y

EXPOSE 3000

ENTRYPOINT ["./tda-server"]
