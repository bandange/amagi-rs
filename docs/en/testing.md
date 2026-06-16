# Amagi-rs Testing Layout

Chinese version: [../中文/测试分层.md](../中文/测试分层.md)

## Public Tests: `crates/amagi-testkit`

`amagi-testkit` is the committed test-only crate. It is a workspace member, has `publish = false`, and is safe to run in CI by default.

Put these tests here:

- public API compatibility tests
- feature matrix tests
- Service API route and JSON field contract tests
- CLI help/version smoke tests
- tests that do not need cookies, do not call real upstream services, and do not contain real response data

Common commands:

```bash
cargo test -p amagi-testkit
cargo test -p amagi-testkit --no-default-features --features catalog --tests
cargo test -p amagi-testkit --no-default-features --features adapters --tests
cargo test -p amagi-testkit --no-default-features --features server --tests
```

## Private Tests: `crates/amagi-private-tests`

`amagi-private-tests` is a local private crate. It is ignored by git, is not a workspace member, and is only for real cookies, real upstream calls, and private fixtures.

Run it locally:

```bash
cargo test --manifest-path crates/amagi-private-tests/Cargo.toml
cargo test --manifest-path crates/amagi-private-tests/Cargo.toml -- --ignored
```

Safety rules:

- Real-data tests must use `#[ignore]` by default.
- Cookies and tokens must come only from environment variables or local `.env.test.local`.
- `.env.test.local`, `private-fixtures/`, HAR files, and raw responses must not be committed.
- Do not print complete responses or write snapshots containing real data.
- Assert structure and behavior instead of storing real payloads as baselines.

Local `.env.test.local` example:

```dotenv
AMAGI_DOUYIN_COOKIE=
AMAGI_PRIVATE_DOUYIN_AWEME_ID=
```

The private crate lives inside the repository only to reuse local path dependencies. Its `Cargo.toml` contains an empty `[workspace]` table so Cargo does not attach it to the root workspace.
