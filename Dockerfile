# API

FROM rust:1-alpine AS builder-api

ENV SQLX_OFFLINE=true

RUN apk update && \
    apk add libressl-dev musl-dev pkgconfig

WORKDIR /app

COPY ./api/ ./api/
COPY ./.sqlx/ ./.sqlx/
COPY ./Cargo.toml ./Cargo.lock .

RUN cargo build --all --release

# WEB

FROM node:22-alpine AS builder-web

ENV PNPM_HOME="/pnpm"
ENV PATH="$PNPM_HOME:$PATH"

RUN corepack enable

WORKDIR /app

COPY ./web/ ./web/
COPY ./package.json ./pnpm-lock.yaml ./pnpm-workspace.yaml .

ENV PUBLIC_API_PATH=/api/

RUN pnpm i --frozen-lockfile && \
    pnpm run --filter=web build

# RELEASE

FROM nginx:alpine AS release

RUN apk add sqlite

WORKDIR /app

COPY ./docker/start.sh .
COPY ./migrations ./migrations
COPY ./docker/nginx.conf /etc/nginx/nginx.conf
COPY --from=builder-api /app/target/release/singarr-api /usr/bin/singarr-api
COPY --from=builder-web /app/web/dist /usr/share/nginx/html

ENV DATABASE_URL=sqlite:/data/db.sqlite
ENV SETTINGS_PATH=/data/settings.json

CMD ["sh", "start.sh"]