FROM rust:1.67.1-slim-buster AS build

WORKDIR networking_accumlator/
RUN apt-get update && apt-get -y install pkg-config openssl libssl-dev --no-install-recommends

COPY Cargo.toml .
# create a dummy file to force cargo to build the dependencies
RUN mkdir src \
   && echo "// dummy file" > src/lib.rs \
   && cargo build --lib

COPY src src
RUN cargo build --release

FROM rust:1.67.1-slim-buster

WORKDIR networking_accumlator/
COPY --from=build networking_accumlator/target/release/networking_accumlator .
# ENTRYPOINT ["./networking_accumlator"]
CMD ["./networking_accumlator"]
EXPOSE 8000
