FROM rust:1.75-slim-buster

RUN mkdir /code

WORKDIR /code

COPY Cargo.toml Cargo.toml
COPY Rocket.toml Rocket.toml
COPY src src

ENTRYPOINT ["cargo", "run", "--release"]
