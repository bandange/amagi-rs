# Publishing

`amagi 0.1.6` was already published as a single-crate structure. The workspace split introduces internal crates that the top-level `amagi` crate depends on.

## Recommended Version

Use `0.2.0` as the first workspace-split release unless the maintainer explicitly chooses a patch release.

Reasons:

- `0.1.6` is the final single-crate-era version.
- Workspace split, feature dependency changes, and release ordering are structural changes.
- Published crates.io versions cannot be overwritten.

## Release Order

Publish internal crates before the facade crate:

```bash
cargo publish -p amagi-core
cargo publish -p amagi-adapters
cargo publish -p amagi-client
cargo publish -p amagi-server
cargo publish -p amagi-cli
cargo publish -p amagi
```

Use the same order for dry runs:

```bash
cargo publish -p amagi-core --dry-run
cargo publish -p amagi-adapters --dry-run
cargo publish -p amagi-client --dry-run
cargo publish -p amagi-server --dry-run
cargo publish -p amagi-cli --dry-run
cargo publish -p amagi --dry-run
```

Wait for the crates.io index to update after each actual publish.

## Preflight

```bash
cargo check --workspace --no-default-features
cargo test -p amagi-testkit
```

## Risks

- Do not publish `amagi` before its registry dependencies exist.
- Keep workspace dependency versions aligned with the release version.
- Never depend from a published crate on a `publish = false` crate.
- If a release has already been published, fix mistakes with a new version or yank; do not try to overwrite.

