FROM rust:1.67.1 AS builder

COPY Cargo.toml Cargo.toml
COPY src src

RUN cargo build

FROM ubuntu:20.04

COPY --from=builder ./target/debug/send-email /tmp/send-email
