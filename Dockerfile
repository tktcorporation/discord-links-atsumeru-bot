FROM rust:1.53-slim-buster AS build-env

RUN apt-get update && \
    apt-get install -y \
    libopus-dev \
    build-essential \
    libssl-dev \
    pkg-config \
    autoconf \
    automake \
    libtool \
    m4 \
    curl \
    git

RUN curl https://raw.githubusercontent.com/nektos/act/master/install.sh | bash

ENV LC_ALL=C.UTF-8

COPY Cargo.lock Cargo.lock
COPY Cargo.toml Cargo.toml

RUN mkdir src/
RUN echo "fn main() {println!(\"if you see this, the build broke\")}" > src/main.rs
RUN cargo build --release
RUN rm -f target/release/deps/discord*

COPY  . .

RUN cargo build --release

CMD [ "/bin/sh",  "-c", "cargo run" ]

FROM debian:buster-20210621-slim

RUN apt-get update && \
    apt-get install -y \
    libopus-dev \
    build-essential \
    libssl-dev \
    pkg-config \
    autoconf \
    automake \
    libtool \
    curl \
 && apt-get -y clean \
 && rm -rf /var/lib/apt/lists/*

ENV LC_ALL=C.UTF-8

COPY --from=build-env /target/release/discord-links-transfer-bot /bin/discord-links-transfer-bot

CMD [ "/bin/sh",  "-c", "discord-links-transfer-bot" ]