# Workspace Structure

Root workspace members:

- `crates/amagi`
- `crates/amagi-core`
- `crates/amagi-adapters`
- `crates/amagi-client`
- `crates/amagi-server`
- `crates/amagi-cli`
- `crates/amagi-testkit`
- `crates/amagi-docs`

`crates/amagi-private-tests` exists as a private test crate and is not a workspace member.

## Publish Policy

Publishable crates:

- `amagi`
- `amagi-core`
- `amagi-adapters`
- `amagi-client`
- `amagi-server`
- `amagi-cli`

Do not publish:

- `amagi-docs`
- `amagi-testkit`
- `amagi-private-tests`

## Crate Roles

- `amagi`: public facade crate and `amagi` binary.
- `amagi-core`: shared types, configuration, and core helpers.
- `amagi-adapters`: platform fetchers, API specs, signing, and adapter logic.
- `amagi-client`: Rust API client facade.
- `amagi-server`: Axum HTTP server and node transport.
- `amagi-cli`: command-line runtime.
- `amagi-testkit`: test support only.
- `amagi-docs`: documentation site only.

## Feature Notes

Default `amagi` features are currently `client`, `cli`, and `server`.

Compatibility aliases:

- `platforms` maps to adapter capability.
- `catalog` maps to API catalog capability.

When changing features, check both the top-level `amagi` feature graph and the implementation crate features.

