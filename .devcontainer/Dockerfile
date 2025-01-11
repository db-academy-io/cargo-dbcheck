FROM rust:1.84

RUN apt-get update && apt-get install -y \
    libdbus-1-dev \
    pkg-config \
    pre-commit
    
RUN rustup component add clippy rustfmt
RUN cargo install just cargo-version-upgrade
