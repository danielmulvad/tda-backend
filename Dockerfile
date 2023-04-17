# Builder
FROM rust:1.68-slim as builder

WORKDIR /usr/src/tda-server

COPY ./Cargo.toml ./Cargo.toml
COPY ./Cargo.lock ./Cargo.lock
COPY ./src/ ./src/

# Set up environment variables
RUN echo "LOG_LEVEL=DEBUG" > .env
RUN echo "TDA_API_BASE_URL=https://tradetracker.dmulvad.com" >> .env
RUN echo "TDA_API_CALLBACK_URL=https://tradetracker.dmulvad.com/api/auth/callback/tda" >> .env
RUN --mount=type=secret,id=TDA_API_KEY awk '{print "\nTDA_API_KEY="$1}' /run/secrets/TDA_API_KEY >> .env
RUN --mount=type=secret,id=FIREBASE_API_KEY awk '{print "\nFIREBASE_API_KEY="$1}' /run/secrets/FIREBASE_API_KEY >> .env
RUN --mount=type=secret,id=CLOUDFLARE_TURNSTILE_SECRET_KEY awk '{print "\nCLOUDFLARE_TURNSTILE_SECRET_KEY="$1}' /run/secrets/CLOUDFLARE_TURNSTILE_SECRET_KEY >> .env
RUN --mount=type=secret,id=JWT_ACCESS_TOKEN_SECRET awk '{print "\nJWT_ACCESS_TOKEN_SECRET="$1}' /run/secrets/JWT_ACCESS_TOKEN_SECRET >> .env
RUN --mount=type=secret,id=JWT_REFRESH_TOKEN_SECRET awk '{print "\nJWT_REFRESH_TOKEN_SECRET="$1}' /run/secrets/JWT_REFRESH_TOKEN_SECRET >> .env

# Install dependencies
RUN apt update && apt install pkg-config openssl libssl-dev ca-certificates -y

# https://planetscale.com/docs/concepts/secure-connections#ca-root-configuration
RUN echo "MYSQL_ATTR_SSL_CA=/etc/ssl/certs/ca-certificates.crt" >> .env

RUN cargo install --path . --target-dir ./target

# Runner
FROM debian:buster-slim

COPY --from=builder /usr/src/tda-server /usr/local/bin/tda-server
COPY --from=builder /usr/src/tda-server/.env /usr/local/bin/tda-server/target/release/.env

WORKDIR /usr/local/bin/tda-server/target/release

RUN apt update && apt install pkg-config openssl libssl-dev ca-certificates -y

EXPOSE 3000

ENTRYPOINT ["./tda-server"]
