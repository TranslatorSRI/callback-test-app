FROM rust:1.63-slim-buster

RUN mkdir /code

WORKDIR /code

COPY Cargo.toml Cargo.toml
COPY Rocket.toml Rocket.toml
COPY src src

RUN cargo build --release

ENTRYPOINT ["cargo", "run", "--release"]
