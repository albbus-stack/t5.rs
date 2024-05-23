FROM rust:1-bookworm as builder

WORKDIR /usr/src/app
COPY .env .env
COPY ./.cargo ./.cargo
COPY ./api .
COPY ./common ../common

RUN apt-get update -qq && apt-get install -y libpq-dev

RUN --mount=type=cache,target=/usr/local/cargo,from=rust:latest,source=/usr/local/cargo \
    --mount=type=cache,target=target \
    cargo build --release && mv ./target/release/api ./api

FROM debian:bookworm-slim

RUN useradd -ms /bin/bash app

USER root
RUN apt-get update -qq && apt-get install -y libpq-dev
USER app

WORKDIR /app
COPY --from=builder /usr/src/app/api /app/api
COPY --from=builder /usr/src/app/.env /app/.env

CMD ./api