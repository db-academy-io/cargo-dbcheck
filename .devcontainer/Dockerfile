FROM rust:1.84

RUN apt-get update && apt-get install -y \
    libdbus-1-dev \
    pkg-config

RUN rustup component add clippy
RUN rustup component add rustfmt
RUN cargo install just
