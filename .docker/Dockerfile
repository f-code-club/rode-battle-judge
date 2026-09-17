FROM lukemathwalker/cargo-chef:latest-rust-latest AS chef
WORKDIR /app

FROM chef AS planner
COPY Cargo.toml Cargo.lock ./
COPY src/ ./src
COPY .sqlx ./.sqlx
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder 
COPY --from=planner /app/recipe.json recipe.json

RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/usr/local/cargo/git \
    cargo chef cook --release --recipe-path recipe.json
COPY Cargo.toml Cargo.lock ./
COPY src/ ./src
COPY .sqlx ./.sqlx
COPY migrations ./migrations
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/usr/local/cargo/git \
    cargo build --release

FROM debian:stable AS application

RUN apt-get update \
    && apt-get install libssl3 ca-certificates chromium openjdk-21-jdk build-essential python3 python-is-python3 libgbm-dev libxkbcommon-x11-0 -y \
    && apt-get clean

WORKDIR /app

COPY --from=builder /app/target/release/rode-battle-judge .

CMD ["./rode-battle-judge"]
