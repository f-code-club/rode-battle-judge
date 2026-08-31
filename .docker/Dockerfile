FROM lukemathwalker/cargo-chef:latest-rust-latest AS chef
WORKDIR /app

FROM chef AS planner
COPY Cargo.toml Cargo.lock ./
COPY src/ ./src
COPY .sqlx ./.sqlx
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder 
COPY --from=planner /app/recipe.json recipe.json

RUN cargo chef cook --release --recipe-path recipe.json
COPY Cargo.toml Cargo.lock ./
COPY src/ ./src
COPY .sqlx ./.sqlx
COPY migrations ./migrations
RUN cargo build --release

FROM debian:stable AS application

RUN apt-get update \
    && apt-get install libssl3 ca-certificates -y \
    && apt-get clean

WORKDIR /app

COPY --from=builder /app/target/release/rode-battle-judge .

CMD ["./rode-battle-judge"]
