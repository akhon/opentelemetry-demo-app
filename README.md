opentelemetry-demo-app
======================

A basic Rust application which generates traces using OpenTelemetry.


## Dependencies

- Redis

## Building

The application can be built using `cargo build --release`. The artifact can be
found at `target/release/opentelemetry-demo-app`.

If trying to build in docker, available base images can be found [on docker
hub](https://hub.docker.com/_/rust)

## Running

```
target/release/opentelemetry-demo-app -f config.yml
```

## Useful environment variables

- `OTEL_EXPORTER_OTLP_ENDPOINT` for configuring the gRPC endpoint where traces
    will be sent.
