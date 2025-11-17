FROM rust:1.91

RUN apt -y update && apt -y install musl-tools libssl-dev pkg-config build-essential mold

RUN rustup update && \
  cargo install bacon cargo-nextest cargo-llvm-cov && \
  rustup component add rustfmt clippy llvm-tools-preview
