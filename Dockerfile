FROM rust:1.83 AS builder

WORKDIR /usr/prom-http-exporter

COPY . .

RUN cargo build --release

FROM debian:stable-slim

WORKDIR /usr/prom-http-exporter

COPY config.toml .

COPY --from=builder /usr/prom-http-exporter/target/release/prom-http-exporter /usr/prom-http-exporter


CMD ["/usr/prom-http-exporter/prom-http-exporter"]