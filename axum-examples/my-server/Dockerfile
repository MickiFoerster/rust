FROM rust:1-bookworm AS builder

WORKDIR /build 

COPY . .

RUN cargo build --release

FROM debian:bookworm-slim 

RUN apt update && \
    apt install -y ca-certificates libssl-dev openssl curl

COPY --from=builder \
    /build/target/release/my-server \
    /usr/local/bin/my-server

ENV EXAMPLE_VAR=some_value_you_can_provide_to_server
ENV RUST_LOG=DEBUG

CMD ["/usr/local/bin/my-server"]
