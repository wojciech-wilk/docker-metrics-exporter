# docker-metrics-exporter

[![Build Status]][Travis]
[![Docker Pulls]][Docker Hub]
[![Rust Version]][Rust]
[![License]][MIT]

**docker-metrics-exporter is a tool to export various Docker metrics in the Prometheus format.**

---

**Features:**

- Provides Docker Swarm metrics
- Uses Docker API
- Small resources footprint

**Why not simply use Docker engine out-of-the-box metrics?**

- Docker engine provides metrics as an experimental feature
  (metrics, names can change without backward compatibility)
- they are limited, especially in terms of the Swarm

## Install

### Precompiled binaries

### Docker images

## Metrics

For the complete list of supported metrics see [METRICS.md](METRICS.md).

## Configuration

Currently the only configuration mode is to use environment variables.

Configuration with defaults:
```bash
RUST_LOG=info,docker_metrics_exporter=debug

# HTTP server bind address that will be scraped by the Prometheus
DME_HTTP_ADDRESS=0.0.0.0
# HTTP server bind port
DME_HTTP_PORT=8080

# Docker daemon URL
# socket
DME_DOCKER_URL=unix:///var/run/docker.sock
# TCP
#DME_DOCKER_URL=http://127.0.0.1:2375
```

## Build

To build `docker-metrics-exporter` you need Rust 1.40+ toolchain.

Use `cargo` to download dependencies and build:

```bash
cargo build --release
```

## License

MIT, see [LICENSE](LICENSE)

[Build status]: https://travis-ci.org/Galhad/docker-metrics-exporter.svg?branch=master
[Travis]: https://travis-ci.org/Galhad/docker-metrics-exporter

[License]: https://img.shields.io/badge/License-MIT-brightgreen.svg
[MIT]: https://opensource.org/licenses/MIT

[Docker Pulls]: https://img.shields.io/docker/pulls/galhad/docker-metrics-exporter.svg?maxAge=604800 
[Docker Hub]: https://hub.docker.com/r/galhad/docker-metrics-exporter

[Rust Version]: https://img.shields.io/badge/rustc-1.39+-lightgray.svg
[Rust]: https://blog.rust-lang.org/2019/11/07/Rust-1.39.0.html
