# Documentation Site

The docs site is `crates/amagi-docs`.

It is a Dioxus static web app and is marked `publish = false`.

## Commands

Serve locally:

```bash
dx serve --package amagi-docs
```

Build static output:

```bash
dx build --package amagi-docs --release
```

Check Rust code:

```bash
cargo check -p amagi-docs
cargo test -p amagi-docs
```

## Content Sources

Home pages:

- English: root `README.md`
- Chinese: root `README.zh-CN.md`

Docs:

- English: `docs/en`
- Chinese: `docs/中文`

Legal pages:

- English: `DISCLAIMER.md`
- Chinese: `DISCLAIMER.zh-CN.md`

## Routing

The app uses Dioxus Router. Markdown links pointing at local `.md` files should be rewritten into Dioxus routes, for example:

- `docs/en/installation.md` -> `/en/installation`
- `docs/中文/参考/命令行参考.md` -> `/zh/cli`
- `README.md` -> `/en`
- `README.zh-CN.md` -> `/zh`

Do not expose raw Markdown files as the main navigation path.

## UI Notes

- Language switching belongs in the top bar.
- Search belongs in the top bar and should show results below the top bar.
- Theme switching is a single icon toggle with persisted browser preference and system default fallback.
- Code blocks use local Highlight.js assets and copy buttons.
- Keep service terminology distinct from documentation-site terminology.

