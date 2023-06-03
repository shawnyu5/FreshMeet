FROM rust:latest AS build

WORKDIR /networking_accumlator/
RUN apt-get update && apt-get -y install pkg-config openssl libssl-dev --no-install-recommends

COPY Cargo.toml .
# create a dummy file to force cargo to build the dependencies
RUN mkdir src \
   && echo "// dummy file" > src/lib.rs \
   && cargo build --lib --release

COPY src src
RUN cargo build --release

FROM rust:latest

WORKDIR /networking_accumlator/
COPY --from=build networking_accumlator/target/release/networking_accumlator .

ENV ROCKET_ADDRESS=0.0.0.0
EXPOSE 8000
# ENTRYPOINT ["./networking_accumlator"]
CMD ["./networking_accumlator"]
