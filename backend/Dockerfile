FROM rust:latest AS build
WORKDIR /fresh_meet/
RUN apt-get update && apt-get -y install pkg-config openssl libssl-dev --no-install-recommends

COPY Cargo.toml .
# COPY rust-toolchain.toml .

# create a dummy file to force cargo to build the dependencies
RUN mkdir src \
   && echo "// dummy file" > src/lib.rs \
   && cargo build --lib --release

COPY src src
RUN cargo build --release

FROM rust:latest

WORKDIR /fresh_meet/
COPY --from=build fresh_meet/target/release/fresh_meet .

# ENV ROCKET_ADDRESS=0.0.0.0
EXPOSE 8000
# ENTRYPOINT ["./networking_accumlator"]
CMD ["./fresh_meet"]
