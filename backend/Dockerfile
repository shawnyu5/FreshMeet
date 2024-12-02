# FROM rust:latest AS build
FROM lukemathwalker/cargo-chef:latest-rust-1 AS chef
WORKDIR /fresh_meet/

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /fresh_meet/recipe.json recipe.json
# Build dependencies - this is the caching Docker layer!
RUN cargo chef cook --release --recipe-path recipe.json
# Build application
COPY . .
RUN cargo build --release --bin fresh_meet

FROM debian:bookworm-slim AS runtime
WORKDIR /fresh_meet/
COPY --from=builder fresh_meet/target/release/fresh_meet .
EXPOSE 8000
CMD ["./fresh_meet"]
