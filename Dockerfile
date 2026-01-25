FROM rust:1-alpine AS base-api

WORKDIR /app

ENV SQLX_OFFLINE=true

RUN cargo install cargo-chef


FROM base-api AS planner-api

COPY ./Cargo.toml ./Cargo.lock ./
COPY ./api ./api

RUN cargo chef prepare --recipe-path recipe.json


FROM base-api AS builder-api

RUN apk update && \
    apk add libressl-dev musl-dev pkgconfig

COPY --from=planner-api /app/recipe.json recipe.json

RUN cargo chef cook --release --recipe-path recipe.json

COPY ./Cargo.toml ./Cargo.lock ./
COPY ./api ./api
COPY ./.sqlx ./.sqlx

RUN cargo build --release


FROM node:22-alpine AS builder-web

WORKDIR /app

ENV PNPM_HOME="/pnpm"
ENV PATH="$PNPM_HOME:$PATH"

RUN corepack enable

COPY ./web/package.json ./web/package.json
COPY ./package.json ./pnpm-lock.yaml ./pnpm-workspace.yaml ./

RUN pnpm i --frozen-lockfile

COPY ./web ./web

ENV PUBLIC_API_PATH=/api/
ENV PUBLIC_IMAGES_PATH=/images/

RUN pnpm run --filter=web build


FROM nginx:alpine AS release

WORKDIR /app

RUN apk add sqlite

COPY ./docker/start.sh .
COPY ./docker/nginx.conf /etc/nginx/nginx.conf
COPY ./migrations ./migrations

COPY --from=builder-api /app/target/release/singarr-api /usr/bin/singarr-api
COPY --from=builder-web /app/web/dist /usr/share/nginx/html

ENV DATABASE_URL=sqlite:/config/db.sqlite
ENV SETTINGS_PATH=/config

CMD ["sh", "start.sh"]
