ARG VARIANT="bookworm"
FROM rust:1-${VARIANT} AS builder

WORKDIR /app

COPY Cargo.toml Cargo.lock ./

COPY src ./src
RUN cargo install --path .

# ======== Final Stage ========
FROM rust:1-${VARIANT} AS final

COPY --from=builder /usr/local/cargo/bin/cargo-dbcheck /usr/local/bin/
ENV PATH="/root/.cargo/bin:${PATH}"
ENV CARGO_FORCE_CARGO_BIN=1 
CMD ["bash"]
