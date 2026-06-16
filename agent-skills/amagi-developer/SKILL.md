---
name: amagi-developer
description: Use this skill when maintaining the Amagi-rs repository, including Rust workspace crate boundaries, feature wiring, publishing order, Dioxus documentation site changes, platform adapter debugging, testing strategy, and release preparation. Use for implementation tasks inside the repository, not for ordinary end-user CLI help.
license: GPL-3.0-only
metadata:
  project: amagi-rs
  audience: developer
---

# Amagi-rs Developer

Maintain the Amagi-rs repository without blurring public API, internal crate, documentation, and test boundaries.

## Source Priority

1. Read the root `Cargo.toml` before changing workspace membership, versions, dependencies, or features.
2. Read the relevant crate `Cargo.toml` before modifying a crate.
3. Read the matching reference only when needed:
   - `references/workspace.md`: crate roles, publish policy, feature boundaries.
   - `references/publishing.md`: crates.io migration and release order.
   - `references/docs-site.md`: Dioxus docs site architecture and Markdown routing.
   - `references/testing.md`: focused verification commands and test crate boundaries.

## Core Rules

1. Preserve `amagi` as the public facade crate and binary command.
2. Keep `Amagi-rs` as the project name in prose.
3. Do not publish documentation or test crates.
4. Keep changes scoped to the crate or layer implied by the task.
5. Prefer existing workspace patterns over new abstractions.
6. For platform adapter fixes, verify whether the issue is fetcher logic, request signing, API shape, cookies, or upstream platform behavior before broad refactoring.

## Common Gotchas

- The split workspace cannot publish only `amagi` if it depends on unpublished internal crates.
- `amagi-docs` is a Dioxus static documentation site, not the service runtime.
- Docs home pages render root `README.md` and `README.zh-CN.md`.
- Chinese documentation filenames should remain Chinese.
- CLI command wording is `amagi`; project wording is `Amagi-rs`.
- Live-stream availability can depend on platform login state, cookies, region, and upstream anti-abuse behavior.

## Validation

Choose focused checks first. Escalate to broader checks when shared crates, public features, or release behavior change.

