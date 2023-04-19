FROM rust:1.68-slim-buster as builder

RUN USER=root

RUN mkdir tradetracker
WORKDIR /tradetracker
ADD . ./

RUN apt update && apt install pkg-config openssl libssl-dev ca-certificates -y
RUN echo "LOG_LEVEL=DEBUG" > .env
RUN echo "TDA_API_BASE_URL=https://tradetracker.dmulvad.com" >> .env
RUN echo "TDA_API_CALLBACK_URL=https://tradetracker.dmulvad.com/api/auth/callback/tda" >> .env
RUN --mount=type=secret,id=TDA_API_KEY awk '{print "\nTDA_API_KEY="$1}' /run/secrets/TDA_API_KEY >> .env
RUN --mount=type=secret,id=FIREBASE_API_KEY awk '{print "\nFIREBASE_API_KEY="$1}' /run/secrets/FIREBASE_API_KEY >> .env
RUN --mount=type=secret,id=CLOUDFLARE_TURNSTILE_SECRET_KEY awk '{print "\nCLOUDFLARE_TURNSTILE_SECRET_KEY="$1}' /run/secrets/CLOUDFLARE_TURNSTILE_SECRET_KEY >> .env
RUN --mount=type=secret,id=JWT_ACCESS_TOKEN_SECRET awk '{print "\nJWT_ACCESS_TOKEN_SECRET="$1}' /run/secrets/JWT_ACCESS_TOKEN_SECRET >> .env
RUN --mount=type=secret,id=JWT_REFRESH_TOKEN_SECRET awk '{print "\nJWT_REFRESH_TOKEN_SECRET="$1}' /run/secrets/JWT_REFRESH_TOKEN_SECRET >> .env
RUN cargo clean && \
    cargo build -vv --release

FROM debian:buster-slim

ARG APP=/usr/src/app

ENV APP_USER=appuser

RUN apt update && apt install pkg-config openssl libssl-dev ca-certificates -y
RUN groupadd $APP_USER \
    && useradd -g $APP_USER $APP_USER \
    && mkdir -p ${APP}

COPY --from=builder /tradetracker/target/release/tda-server ${APP}/tda-server

RUN chown -R $APP_USER:$APP_USER ${APP}

USER $APP_USER
WORKDIR ${APP}

CMD ["./tda-server"]