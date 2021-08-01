# select image
FROM rust:latest

WORKDIR /usr/src/crypto-payments

# copy your source tree
COPY ./ ./

# build for release
RUN cargo build --release
RUN cargo install --path .
RUN cargo install diesel_cli --no-default-features --features sqlite

RUN ["chmod", "+x", "./entrypoint.sh"]
ENTRYPOINT ["./entrypoint.sh"]