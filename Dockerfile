FROM rust:1.83.0 AS builder

WORKDIR /usr/prom-http-exporter

COPY . .

RUN cargo build --release

FROM debian:stable-slim

WORKDIR /usr/prom-http-exporter

COPY --from=builder /usr/prom-http-exporter/target/release/prom-http-exporter /exe/prom-http-exporter

COPY config.toml /etc/prom-http-exporter/config.toml

RUN chown -R nobody:root /etc/prom-http-exporter /exe

USER nobody

ENTRYPOINT ["/exe/prom-http-exporter"]
CMD [ "/etc/prom-http-exporter/config.toml" ]