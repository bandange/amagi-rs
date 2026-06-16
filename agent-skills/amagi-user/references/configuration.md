# Configuration

Amagi-rs loads configuration in this order:

1. The user-level dotenv file.
2. The current directory `.env`.

User-level dotenv path:

- Linux/macOS: `~/.config/amagi/.env`
- Windows: `%APPDATA%\\amagi\\.env`

Useful files:

- `.env.example`
- `.env.example.zh-CN`

Common variables:

- `AMAGI_USER_ENV_FILE`
- `AMAGI_DOUYIN_COOKIE`
- `AMAGI_BILIBILI_COOKIE`
- `AMAGI_KUAISHOU_COOKIE`
- `AMAGI_TWITTER_COOKIE`
- `AMAGI_XIAOHONGSHU_COOKIE`
- `AMAGI_HOST`
- `AMAGI_PORT`

## Cookie Guidance

Some platform features may work without login, while others require a valid cookie. Live-room and live-stream data are especially sensitive to platform-side login, region, and anti-abuse checks.

When troubleshooting cookie issues:

1. Confirm whether a platform-specific cookie variable exists.
2. Do not ask the user to share raw cookie values.
3. Ask for sanitized cookie key presence, command output, and target identifier.
4. Note that third-party platform behavior can change without an Amagi-rs release.

## Responsible Use

Always remind users to comply with applicable laws, platform terms, rate limits, account rules, and third-party rights. Amagi-rs is not an official client for third-party services.

