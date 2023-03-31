FROM rust:1.68-slim-buster as build
RUN apt-get update && apt-get -y install pkg-config libssl-dev

RUN USER=root cargo new store-core
WORKDIR /store-core

COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml
COPY ./src ./src

RUN cargo build --release

# our final base
FROM debian:buster-slim

RUN apt-get update && apt-get -y install pkg-config libssl-dev
RUN apt-get install -y ca-certificates

COPY --from=build /store-core/target/release/store-core .

CMD  ["./store-core"]