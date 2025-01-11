ARG VARIANT="bullseye"
FROM rust:1-${VARIANT} AS builder

WORKDIR /app

RUN apt-get update && apt-get install -y libpq-dev pkg-config libdbus-1-dev
COPY Cargo.toml Cargo.lock ./

COPY src ./src
RUN cargo install --path .

# ======== Final Stage ========
FROM rust:1-${VARIANT} AS final

COPY --from=builder /usr/local/cargo/bin/cargo-dbcheck /usr/local/bin/
RUN apt-get update && apt-get install -y libpq-dev pkg-config libdbus-1-dev
ENV PATH="/usr/local/bin:${PATH}"
ENV CARGO_FORCE_CARGO_BIN=1 
CMD ["bash"]
