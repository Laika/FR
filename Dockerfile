# Build
FROM rust:1.59 as builder

WORKDIR /usr/src/fr
COPY . .
RUN cargo clean && cargo install --path .


# Run
FROM ubuntu:focal
RUN apt-get update && rm -rf /var/lib/apt/lists/*

COPY --from=builder /usr/local/cargo/bin/fr /usr/local/bin/fr
CMD ["fr"]
