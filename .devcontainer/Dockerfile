FROM rust:1.84

RUN apt-get update && apt-get install -y \
    libdbus-1-dev \
    pkg-config

RUN cargo install just
