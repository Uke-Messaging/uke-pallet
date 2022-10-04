FROM rust:latest
WORKDIR /usr/src/uke

COPY . .

# Runs pallet unit tests, including runtime-benchmarks
RUN cargo test --package pallet-uke --features runtime-benchmarks