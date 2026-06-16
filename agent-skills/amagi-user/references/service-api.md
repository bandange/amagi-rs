# Local JSON API Service

Use `amagi serve` to start the local service runtime:

```bash
amagi serve --host 127.0.0.1 --port 4567
```

From source:

```bash
cargo run -p amagi -- serve --host 127.0.0.1 --port 4567
```

## Terminology

Describe this mode as:

- local JSON API service
- service runtime
- HTTP service

Avoid describing it as a website or webpage. The Dioxus documentation site is separate from the `amagi serve` runtime.

## Configuration

The service can use:

- `AMAGI_HOST`
- `AMAGI_PORT`
- Platform cookie variables such as `AMAGI_KUAISHOU_COOKIE`

When users report service failures, ask for:

1. The exact startup command.
2. The local URL and endpoint they called.
3. The sanitized error response.
4. Whether CLI usage for the same platform target succeeds.

