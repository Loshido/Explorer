FROM oven/bun:alpine AS web
WORKDIR /app

COPY ./web /app/
RUN bun i && bun run build

FROM rust:alpine AS bin
WORKDIR /app


COPY src/ /app/src/
COPY Cargo.* .
COPY Rocket.toml .

RUN apk add build-base && cargo build --release

FROM alpine:latest
WORKDIR /app

COPY --from=web /dist /app/dist
COPY --from=bin /app/target/release/explorer /app/explorer
COPY Rocket.toml .

ARG ROCKET_ADDRESS=0.0.0.0
ARG ROCKET_PORT=80
ARG ROCKET_LOG_LEVEL=normal

EXPOSE ${ROCKET_PORT}
ENTRYPOINT [ "/app/explorer" ]