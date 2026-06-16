# Testing

Prefer the narrowest useful verification command.

## Documentation Site

```bash
cargo check -p amagi-docs
cargo test -p amagi-docs
```

Use a browser or Playwright-style check for visual or routing changes:

- `/zh`
- `/en`
- `/zh/disclaimer`
- `/en/disclaimer`

## Workspace

For shared crate, feature, or release changes:

```bash
cargo check --workspace --no-default-features
cargo test -p amagi-testkit
```

## Platform Adapter Work

For platform-specific live data issues:

1. Check whether cookies exist and are loaded.
2. Test unauthenticated behavior separately from authenticated behavior.
3. Capture sanitized request/response details.
4. Decide whether the failure is parser shape drift, request signing, endpoint changes, login gating, region gating, or network/proxy behavior.

Never commit private cookies, raw platform responses containing private data, or local `.env` secrets.

