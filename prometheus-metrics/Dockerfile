FROM debian

COPY target/debug/example-prometheus-metrics \
    /usr/local/bin/prometheus-playground

ENV RUST_LOG=debug

CMD ["/usr/local/bin/prometheus-playground"]
