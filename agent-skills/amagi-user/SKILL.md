---
name: amagi-user
description: Use this skill when helping users install, configure, run, or troubleshoot Amagi-rs as a CLI, Rust API, or local JSON API service. Covers command usage, environment variables, cookies, service startup, output interpretation, platform capability limits, and responsible-use disclaimers.
license: GPL-3.0-only
metadata:
  project: amagi-rs
  audience: user
---

# Amagi-rs User

Help users operate Amagi-rs. Keep the answer focused on usage, setup, runtime configuration, and troubleshooting. Do not expose repository-maintenance details unless the user asks to build or modify the project from source.

## Source Priority

1. Prefer the repository root `README.md` or `README.zh-CN.md` for current quick-start commands.
2. Use `docs/en` or `docs/中文` for language-specific docs when available.
3. Read the reference files in this skill only when the task matches their scope:
   - `references/cli.md`: CLI commands, examples, and command semantics.
   - `references/configuration.md`: cookies, environment variables, dotenv loading, and troubleshooting.
   - `references/service-api.md`: `amagi serve`, local JSON API service usage, and service terminology.

## Working Style

1. Identify whether the user wants CLI, Rust API, or local service usage.
2. Give commands that use the public `amagi` command name.
3. Use `Amagi-rs` for project/product wording and `amagi` only for the command or crate name.
4. Prefer install-script usage for end users. Mention source builds only when the user asks to avoid installing binaries or wants development setup.
5. Include responsible-use reminders when the task touches third-party platform data, cookies, live streams, scraping, or authentication.

## Troubleshooting Checklist

For failures, check in this order:

1. Confirm the installed version with `amagi --version`.
2. Confirm the exact command and platform subcommand.
3. Check whether the platform endpoint requires cookies or login state.
4. Check whether relevant `AMAGI_*_COOKIE` values are loaded from the expected dotenv file.
5. Ask for the sanitized error output, command, platform target, and whether proxy/network restrictions apply.

Never ask users to paste raw private cookies into public logs. Ask for sanitized values or only the presence/absence of required cookie keys.
