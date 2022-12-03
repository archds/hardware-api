FROM rust:1.65 as build

RUN USER=root cargo new --bin hardware-api
WORKDIR /hardware-api

COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

RUN cargo build --release
RUN rm src/*.rs

COPY ./Rocket.toml ./Rocket.toml
COPY ./src ./src

RUN rm ./target/release/deps/hardware_api*
RUN cargo build --release


FROM debian:buster-slim

COPY --from=build /hardware-api/target/release/hardware-api .

CMD ["./hardware-api"]
