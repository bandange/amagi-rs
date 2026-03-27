#!/usr/bin/env bash
set -euo pipefail

BIN_NAME="amagi"
DEFAULT_SOURCE_MODE="${AMAGI_INSTALL_SOURCE:-auto}"
REMOTE_REPO_OWNER="${AMAGI_REMOTE_REPO_OWNER:-bandange}"
REMOTE_REPO_NAME="${AMAGI_REMOTE_REPO_NAME:-amagi-rs}"
REMOTE_VERSION="${AMAGI_INSTALL_VERSION:-latest}"
SCRIPT_PATH="${BASH_SOURCE[0]:-}"
SCRIPT_DIR=""
REPO_ROOT=""

if [[ -n "${SCRIPT_PATH}" ]]; then
  SCRIPT_DIR="$(cd -- "$(dirname -- "${SCRIPT_PATH}")" && pwd)"
  REPO_ROOT="$(cd -- "${SCRIPT_DIR}/.." 2>/dev/null && pwd || true)"
fi

default_install_dir() {
  printf '%s\n' "${HOME}/.local/bin"
}

resolve_user_env_file() {
  if [[ -n "${AMAGI_USER_ENV_FILE:-}" ]]; then
    printf '%s\n' "${AMAGI_USER_ENV_FILE}"
    return 0
  fi

  printf '%s\n' "${HOME}/.config/amagi/.env"
}

resolve_project_env_source() {
  if [[ -f "${PWD}/.env" ]]; then
    printf '%s\n' "${PWD}/.env"
    return 0
  fi

  if [[ -n "${REPO_ROOT}" && -f "${REPO_ROOT}/.env" ]]; then
    printf '%s\n' "${REPO_ROOT}/.env"
    return 0
  fi

  return 1
}

absolute_file_path() {
  local path="$1"
  local dir

  dir="$(cd -- "$(dirname -- "${path}")" && pwd)"
  printf '%s/%s\n' "${dir}" "$(basename -- "${path}")"
}

resolve_profile_file() {
  if [[ -n "${AMAGI_PROFILE_FILE:-}" ]]; then
    printf '%s\n' "${AMAGI_PROFILE_FILE}"
    return 0
  fi

  case "${SHELL##*/}" in
    bash)
      printf '%s\n' "${HOME}/.bashrc"
      ;;
    zsh)
      printf '%s\n' "${HOME}/.zshrc"
      ;;
    *)
      printf '%s\n' "${HOME}/.profile"
      ;;
  esac
}

is_sourced() {
  [[ "${BASH_SOURCE[0]}" != "${0}" ]]
}

persist_path_entry() {
  local install_dir="$1"
  local profile_file
  local profile_dir
  local path_line

  profile_file="$(resolve_profile_file)"
  profile_dir="$(dirname -- "${profile_file}")"
  path_line="export PATH=\"${install_dir}:\$PATH\""

  mkdir -p "${profile_dir}"
  touch "${profile_file}"

  if grep -Fqx "${path_line}" "${profile_file}"; then
    printf '[amagi] PATH entry already exists in %s\n' "${profile_file}"
  else
    {
      printf '\n# amagi installer\n'
      printf '%s\n' "${path_line}"
    } >> "${profile_file}"
    printf '[amagi] added install directory to %s\n' "${profile_file}"
  fi

  if [[ ":${PATH}:" == *":${install_dir}:"* ]]; then
    return 0
  fi

  if is_sourced; then
    export PATH="${install_dir}:${PATH}"
    printf '[amagi] updated PATH in the current shell session\n'
  else
    printf '[amagi] restart your shell or run the following command to refresh PATH now:\n'
    printf '  source "%s"\n' "${profile_file}"
  fi
}

sync_user_env_file() {
  local source_path="${1:-}"
  local user_env_path
  local user_env_dir
  local temp_output
  local line
  local key
  local source_count=0
  declare -A source_lines=()
  declare -A seen_keys=()
  local source_keys=()
  local updated_lines=()

  if [[ -z "${source_path}" || ! -f "${source_path}" ]]; then
    printf '[amagi] no project .env found in the current directory; skipped user env sync\n'
    return 0
  fi

  while IFS= read -r line || [[ -n "${line}" ]]; do
    if [[ "${line}" =~ ^[[:space:]]*(export[[:space:]]+)?(AMAGI_[A-Z0-9_]+)[[:space:]]*= ]]; then
      key="${BASH_REMATCH[2]}"
      if [[ -z "${source_lines[$key]+x}" ]]; then
        source_keys+=("${key}")
        source_count=$((source_count + 1))
      fi
      source_lines["${key}"]="${line}"
    fi
  done < "${source_path}"

  if [[ "${source_count}" -eq 0 ]]; then
    printf '[amagi] no AMAGI_* keys found in %s; skipped user env sync\n' "${source_path}"
    return 0
  fi

  user_env_path="$(resolve_user_env_file)"
  user_env_dir="$(dirname -- "${user_env_path}")"
  mkdir -p "${user_env_dir}"

  if [[ -f "${user_env_path}" ]]; then
    while IFS= read -r line || [[ -n "${line}" ]]; do
      if [[ "${line}" =~ ^[[:space:]]*(export[[:space:]]+)?(AMAGI_[A-Z0-9_]+)[[:space:]]*= ]]; then
        key="${BASH_REMATCH[2]}"
        if [[ -n "${source_lines[$key]+x}" ]]; then
          updated_lines+=("${source_lines[$key]}")
          seen_keys["${key}"]=1
        else
          updated_lines+=("${line}")
        fi
      else
        updated_lines+=("${line}")
      fi
    done < "${user_env_path}"
  fi

  for key in "${source_keys[@]}"; do
    if [[ -z "${seen_keys[$key]+x}" ]]; then
      updated_lines+=("${source_lines[$key]}")
    fi
  done

  temp_output="$(mktemp "${TMPDIR:-/tmp}/amagi-env.XXXXXX")"

  {
    for line in "${updated_lines[@]}"; do
      printf '%s\n' "${line}"
    done
  } > "${temp_output}"

  mv "${temp_output}" "${user_env_path}"
  printf '[amagi] synced %s AMAGI_* entries to %s\n' "${source_count}" "${user_env_path}"
}

resolve_execution_mode() {
  case "${DEFAULT_SOURCE_MODE}" in
    auto|local|remote)
      ;;
    *)
      printf '[amagi] unsupported install source mode: %s\n' "${DEFAULT_SOURCE_MODE}" >&2
      exit 1
      ;;
  esac

  if [[ "${DEFAULT_SOURCE_MODE}" != "auto" ]]; then
    printf '%s\n' "${DEFAULT_SOURCE_MODE}"
    return 0
  fi

  if [[ -n "${SCRIPT_DIR}" && -f "${SCRIPT_DIR}/${BIN_NAME}" ]]; then
    printf 'local\n'
    return 0
  fi

  if [[ -n "${REPO_ROOT}" && -f "${REPO_ROOT}/Cargo.toml" ]]; then
    printf 'local\n'
    return 0
  fi

  if [[ -n "${REPO_ROOT}" && -f "${REPO_ROOT}/target/release/${BIN_NAME}" ]]; then
    printf 'local\n'
    return 0
  fi

  printf 'remote\n'
}

resolve_local_binary() {
  local candidates=(
    "${SCRIPT_DIR}/${BIN_NAME}"
    "${REPO_ROOT}/target/release/${BIN_NAME}"
    "${REPO_ROOT}/target/debug/${BIN_NAME}"
  )

  local candidate
  for candidate in "${candidates[@]}"; do
    if [[ -n "${candidate}" && -f "${candidate}" ]]; then
      printf '%s\n' "${candidate}"
      return 0
    fi
  done

  return 1
}

has_repository_workspace() {
  [[ -n "${REPO_ROOT}" && -f "${REPO_ROOT}/Cargo.toml" ]]
}

build_local_release_binary() {
  if ! command -v cargo >/dev/null 2>&1; then
    return 1
  fi

  if [[ -z "${REPO_ROOT}" || ! -f "${REPO_ROOT}/Cargo.toml" ]]; then
    return 1
  fi

  printf '[amagi] no local binary found, building release binary with cargo\n'
  (
    cd "${REPO_ROOT}"
    cargo build --release
  )

  if [[ -f "${REPO_ROOT}/target/release/${BIN_NAME}" ]]; then
    printf '%s\n' "${REPO_ROOT}/target/release/${BIN_NAME}"
    return 0
  fi

  return 1
}

platform_slug() {
  case "$(uname -s)" in
    Linux)
      printf 'linux\n'
      ;;
    Darwin)
      printf 'macos\n'
      ;;
    *)
      printf '[amagi] unsupported operating system for remote install: %s\n' "$(uname -s)" >&2
      exit 1
      ;;
  esac
}

arch_slug() {
  case "$(uname -m)" in
    x86_64|amd64)
      printf 'x86_64\n'
      ;;
    aarch64|arm64)
      printf 'aarch64\n'
      ;;
    *)
      printf '[amagi] unsupported architecture for remote install: %s\n' "$(uname -m)" >&2
      exit 1
      ;;
  esac
}

remote_asset_name() {
  printf '%s-%s-%s\n' "${BIN_NAME}" "$(platform_slug)" "$(arch_slug)"
}

remote_download_url() {
  local asset_name

  asset_name="$(remote_asset_name)"

  if [[ -n "${AMAGI_REMOTE_BASE_URL:-}" ]]; then
    printf '%s/%s\n' "${AMAGI_REMOTE_BASE_URL%/}" "${asset_name}"
    return 0
  fi

  if [[ -z "${REMOTE_REPO_OWNER}" || -z "${REMOTE_REPO_NAME}" ]]; then
    printf '[amagi] remote repository configuration is empty.\n' >&2
    printf '[amagi] set AMAGI_REMOTE_REPO_OWNER and AMAGI_REMOTE_REPO_NAME, or edit scripts/install.sh before using remote install.\n' >&2
    return 1
  fi

  if [[ "${REMOTE_VERSION}" == "latest" ]]; then
    printf 'https://github.com/%s/%s/releases/latest/download/%s\n' \
      "${REMOTE_REPO_OWNER}" "${REMOTE_REPO_NAME}" "${asset_name}"
  else
    printf 'https://github.com/%s/%s/releases/download/%s/%s\n' \
      "${REMOTE_REPO_OWNER}" "${REMOTE_REPO_NAME}" "${REMOTE_VERSION}" "${asset_name}"
  fi
}

download_remote_binary() {
  local url
  local download_path

  url="$(remote_download_url)" || return 1
  download_path="$(mktemp "${TMPDIR:-/tmp}/amagi-install.XXXXXX")"

  printf '[amagi] downloading %s\n' "${url}"

  if command -v curl >/dev/null 2>&1; then
    curl -fsSL "${url}" -o "${download_path}"
  elif command -v wget >/dev/null 2>&1; then
    wget -qO "${download_path}" "${url}"
  else
    printf '[amagi] curl or wget is required for remote install.\n' >&2
    rm -f "${download_path}"
    return 1
  fi

  chmod 755 "${download_path}"
  printf '%s\n' "${download_path}"
}

INSTALL_DIR="${AMAGI_INSTALL_DIR:-$(default_install_dir)}"
INSTALL_MODE="$(resolve_execution_mode)"
SOURCE_BINARY=""

if [[ "${INSTALL_MODE}" == "local" ]]; then
  if [[ -n "${SCRIPT_DIR}" && -f "${SCRIPT_DIR}/${BIN_NAME}" ]]; then
    SOURCE_BINARY="${SCRIPT_DIR}/${BIN_NAME}"
  elif has_repository_workspace; then
    SOURCE_BINARY="$(build_local_release_binary || true)"

    if [[ -z "${SOURCE_BINARY}" ]]; then
      SOURCE_BINARY="$(resolve_local_binary || true)"
    fi
  else
    SOURCE_BINARY="$(resolve_local_binary || true)"
  fi

  if [[ -z "${SOURCE_BINARY}" ]]; then
    printf '[amagi] no local binary found next to the script or in target/release.\n' >&2
    printf '[amagi] remote download is available only when this script runs in remote mode.\n' >&2
    exit 1
  fi
else
  SOURCE_BINARY="$(download_remote_binary || true)"
  if [[ -z "${SOURCE_BINARY}" ]]; then
    exit 1
  fi
fi

mkdir -p "${INSTALL_DIR}"
INSTALL_PATH="${INSTALL_DIR}/${BIN_NAME}"

if [[ "$(absolute_file_path "${SOURCE_BINARY}")" != "$(absolute_file_path "${INSTALL_PATH}")" ]]; then
  cp "${SOURCE_BINARY}" "${INSTALL_PATH}"
fi

chmod 755 "${INSTALL_PATH}"

if [[ "${INSTALL_MODE}" == "remote" && -f "${SOURCE_BINARY}" ]]; then
  rm -f "${SOURCE_BINARY}"
fi

printf '[amagi] installed to %s\n' "${INSTALL_PATH}"
persist_path_entry "${INSTALL_DIR}"
sync_user_env_file "$(resolve_project_env_source || true)"
