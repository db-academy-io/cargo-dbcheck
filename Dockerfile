ARG VARIANT="bookworm"
FROM rust:1-${VARIANT} as builder

WORKDIR /app

COPY Cargo.toml Cargo.lock ./

COPY src ./src
RUN cargo install --path .
ENV PATH="/root/.cargo/bin:${PATH}"

# ======== Final Stage ========
FROM rust:1-${VARIANT} as final

COPY --from=builder /usr/local/cargo/bin/cargo-dbcheck /usr/local/cargo/bin/
CMD ["bash"]
