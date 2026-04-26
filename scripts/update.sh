#!/usr/bin/env bash
set -euo pipefail

DEFAULT_SOURCE_MODE="${AMAGI_UPDATE_SOURCE:-remote}"
DEFAULT_INSTALL_SCRIPT_REF="${AMAGI_INSTALL_SCRIPT_REF:-main}"
REMOTE_REPO_OWNER="${AMAGI_REMOTE_REPO_OWNER:-bandange}"
REMOTE_REPO_NAME="${AMAGI_REMOTE_REPO_NAME:-amagi-rs}"
PROXY_PREFIX="${AMAGI_PROXY_PREFIX:-}"
SCRIPT_PATH="${BASH_SOURCE[0]:-}"
SCRIPT_DIR=""

if [[ -n "${SCRIPT_PATH}" ]]; then
  SCRIPT_DIR="$(cd -- "$(dirname -- "${SCRIPT_PATH}")" && pwd)"
fi

default_proxy_prefix() {
  printf '%s\n' 'https://gh-proxy.com/'
}

normalize_proxy_prefix() {
  local value="${1:-}"

  if [[ -z "${value}" ]]; then
    return 0
  fi

  case "${value}" in
    */)
      printf '%s\n' "${value}"
      ;;
    *)
      printf '%s/\n' "${value}"
      ;;
  esac
}

proxied_url() {
  local url="$1"

  if [[ -n "${PROXY_PREFIX}" ]]; then
    printf '%s%s\n' "${PROXY_PREFIX}" "${url}"
    return 0
  fi

  printf '%s\n' "${url}"
}

require_flag_value() {
  local flag="$1"
  local value="${2:-}"

  if [[ $# -lt 2 || -z "${value}" || "${value}" == --* ]]; then
    printf '[amagi] %s requires a value\n' "${flag}" >&2
    exit 1
  fi
}

resolve_install_script_url() {
  local url

  if [[ -n "${AMAGI_INSTALL_SCRIPT_URL:-}" ]]; then
    printf '%s\n' "${AMAGI_INSTALL_SCRIPT_URL}"
    return 0
  fi

  url="$(printf 'https://raw.githubusercontent.com/%s/%s/%s/scripts/install.sh' \
    "${REMOTE_REPO_OWNER}" \
    "${REMOTE_REPO_NAME}" \
    "${DEFAULT_INSTALL_SCRIPT_REF}")"

  proxied_url "${url}"
}

SOURCE_MODE="${DEFAULT_SOURCE_MODE}"

while [[ $# -gt 0 ]]; do
  case "$1" in
    --source)
      require_flag_value "$@"
      SOURCE_MODE="$2"
      shift 2
      ;;
    --version)
      require_flag_value "$@"
      export AMAGI_INSTALL_VERSION="$2"
      shift 2
      ;;
    --install-dir)
      require_flag_value "$@"
      export AMAGI_INSTALL_DIR="$2"
      shift 2
      ;;
    --proxy)
      PROXY_PREFIX="$(default_proxy_prefix)"
      shift
      ;;
    --proxy-prefix)
      require_flag_value "$@"
      PROXY_PREFIX="$2"
      shift 2
      ;;
    --help|-h)
      cat <<'EOF'
Usage: bash scripts/update.sh [--source remote|local] [--version VERSION] [--install-dir DIR] [--proxy] [--proxy-prefix URL]

Defaults:
  --source remote    Update to the latest published release

Environment passthrough:
  AMAGI_INSTALL_DIR
  AMAGI_INSTALL_VERSION
  AMAGI_PROFILE_FILE
  AMAGI_USER_ENV_FILE
  AMAGI_REMOTE_REPO_OWNER
  AMAGI_REMOTE_REPO_NAME
  AMAGI_REMOTE_BASE_URL
  AMAGI_PROXY_PREFIX
  AMAGI_INSTALL_SCRIPT_URL
  AMAGI_INSTALL_SCRIPT_REF
EOF
      exit 0
      ;;
    *)
      printf '[amagi] unknown flag: %s\n' "$1" >&2
      exit 1
      ;;
  esac
done

PROXY_PREFIX="$(normalize_proxy_prefix "${PROXY_PREFIX}")"

if [[ -n "${PROXY_PREFIX}" ]]; then
  export AMAGI_PROXY_PREFIX="${PROXY_PREFIX}"
else
  unset AMAGI_PROXY_PREFIX || true
fi

case "${SOURCE_MODE}" in
  local|remote)
    ;;
  *)
    printf '[amagi] unsupported update source mode: %s\n' "${SOURCE_MODE}" >&2
    exit 1
    ;;
esac

export AMAGI_INSTALL_SOURCE="${SOURCE_MODE}"

run_local_install_script() {
  local install_script="${SCRIPT_DIR}/install.sh"

  if [[ -n "${SCRIPT_DIR}" && -f "${install_script}" ]]; then
    printf '[amagi] updating via local install script (%s mode)\n' "${AMAGI_INSTALL_SOURCE}"
    exec bash "${install_script}"
  fi

  return 1
}

run_remote_install_script() {
  local install_script_url

  install_script_url="$(resolve_install_script_url)"
  printf '[amagi] updating via %s\n' "${install_script_url}"

  if command -v curl >/dev/null 2>&1; then
    curl -fsSL "${install_script_url}" | bash
    return 0
  fi

  if command -v wget >/dev/null 2>&1; then
    wget -qO- "${install_script_url}" | bash
    return 0
  fi

  printf '[amagi] curl or wget is required for remote update.\n' >&2
  return 1
}

if [[ "${AMAGI_INSTALL_SOURCE}" == "local" ]]; then
  if ! run_local_install_script; then
    printf '[amagi] local update requested but scripts/install.sh is not available next to update.sh.\n' >&2
    exit 1
  fi
fi

if [[ -n "${SCRIPT_DIR}" && -f "${SCRIPT_DIR}/install.sh" ]]; then
  printf '[amagi] updating via local install script (%s mode)\n' "${AMAGI_INSTALL_SOURCE}"
  exec bash "${SCRIPT_DIR}/install.sh"
fi

run_remote_install_script
