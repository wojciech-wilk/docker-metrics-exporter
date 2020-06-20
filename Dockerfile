# === Builder ===
FROM alpine:3.12 as builder
WORKDIR /tmp/docker-metrics-exporter

RUN apk add rust cargo && mkdir -p ./src && echo "fn main() {print!(\"foo\");}" > ./src/main.rs
COPY ./Cargo.toml ./Cargo.toml
# cache dependencies
RUN cargo build --release

COPY ./src ./src
# Force rebuild
RUN touch -t 200001010000 ./target/release/docker-metrics-exporter && \
    cargo build --release

# === Image ===
FROM alpine:3.12
RUN apk add libgcc
COPY --from=builder /tmp/docker-metrics-exporter/target/release/docker-metrics-exporter /docker-metrics-exporter

EXPOSE 8080

ENV RUST_LOG=INFO

CMD ["/docker-metrics-exporter"]
