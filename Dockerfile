# Dockerfile for the PDFium-rs project
#
# This Dockerfile sets up a Rust environment to build and test the
# PDFium-rs library on older Rust versions. Rust 1.80 is the minimum
# version required for this project.

FROM rust:1.80-slim

WORKDIR /app

COPY Cargo.toml .
COPY src ./src
COPY resources ./resources
COPY libpdfium.so .
COPY README.md .

RUN cargo build

RUN cargo test

CMD ["cargo", "test"]