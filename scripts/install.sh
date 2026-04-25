#!/usr/bin/env bash
set -euo pipefail

BIN_NAME="amagi"
DEFAULT_SOURCE_MODE="${AMAGI_INSTALL_SOURCE:-auto}"
INSTALL_DIR="${AMAGI_INSTALL_DIR:-}"
REMOTE_REPO_OWNER="${AMAGI_REMOTE_REPO_OWNER:-bandange}"
REMOTE_REPO_NAME="${AMAGI_REMOTE_REPO_NAME:-amagi-rs}"
REMOTE_VERSION="${AMAGI_INSTALL_VERSION:-latest}"
PROXY_PREFIX=""
REMOTE_DOWNLOAD_PATH=""
REMOTE_EXTRACT_DIR=""
SCRIPT_PATH="${BASH_SOURCE[0]:-}"
SCRIPT_DIR=""
REPO_ROOT=""
AMAGI_MANAGED_BLOCK_START="# >>> amagi installer >>>"
AMAGI_MANAGED_BLOCK_END="# <<< amagi installer <<<"
AMAGI_LEGACY_COMMENT="# amagi installer"
AMAGI_ENV_KEYS=()
declare -A AMAGI_ENV_VALUES=()

if [[ -n "${SCRIPT_PATH}" ]]; then
  SCRIPT_DIR="$(cd -- "$(dirname -- "${SCRIPT_PATH}")" && pwd)"
  REPO_ROOT="$(cd -- "${SCRIPT_DIR}/.." 2>/dev/null && pwd || true)"
fi

require_flag_value() {
  local flag="$1"
  local value="${2:-}"

  if [[ $# -lt 2 || -z "${value}" || "${value}" == --* ]]; then
    printf '[amagi] %s requires a value\n' "${flag}" >&2
    exit 1
  fi
}

while [[ $# -gt 0 ]]; do
  case "$1" in
    --source)
      require_flag_value "$@"
      DEFAULT_SOURCE_MODE="$2"
      shift 2
      ;;
    --install-dir)
      require_flag_value "$@"
      INSTALL_DIR="$2"
      shift 2
      ;;
    --version)
      require_flag_value "$@"
      REMOTE_VERSION="$2"
      shift 2
      ;;
    --proxy)
      PROXY_PREFIX="https://gh-proxy.com/"
      shift
      ;;
    *)
      printf '[amagi] unknown flag: %s\n' "$1" >&2
      exit 1
      ;;
  esac
done

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

resolve_shell_integration_dir() {
  printf '%s\n' "$(dirname -- "$(resolve_user_env_file)")"
}

resolve_posix_shell_file() {
  printf '%s\n' "$(resolve_shell_integration_dir)/shell.sh"
}

resolve_posix_env_export_file() {
  printf '%s\n' "$(resolve_shell_integration_dir)/env.sh"
}

resolve_fish_conf_file() {
  printf '%s\n' "${HOME}/.config/fish/conf.d/amagi.fish"
}

resolve_fish_env_export_file() {
  printf '%s\n' "$(resolve_shell_integration_dir)/env.fish"
}

profile_files() {
  if [[ -n "${AMAGI_PROFILE_FILE:-}" ]]; then
    printf '%s\n' "${AMAGI_PROFILE_FILE}"
    return 0
  fi

  printf '%s\n' "${HOME}/.bashrc"
  printf '%s\n' "${HOME}/.zshrc"
  printf '%s\n' "${HOME}/.profile"
}

is_sourced() {
  [[ "${BASH_SOURCE[0]}" != "${0}" ]]
}

trim_leading_whitespace() {
  local value="$1"
  value="${value#"${value%%[![:space:]]*}"}"
  printf '%s' "${value}"
}

trim_trailing_whitespace() {
  local value="$1"
  value="${value%"${value##*[![:space:]]}"}"
  printf '%s' "${value}"
}

trim_whitespace() {
  local value
  value="$(trim_leading_whitespace "$1")"
  trim_trailing_whitespace "${value}"
}

decode_amagi_env_value() {
  local raw_value="$1"
  local value
  local placeholder=$'\001'

  value="$(trim_whitespace "${raw_value%$'\r'}")"

  if [[ "${#value}" -ge 2 && "${value:0:1}" == '"' && "${value: -1}" == '"' ]]; then
    value="${value:1:${#value}-2}"
    value="${value//\\\\/${placeholder}}"
    value="${value//\\n/$'\n'}"
    value="${value//\\r/$'\r'}"
    value="${value//\\t/$'\t'}"
    value="${value//\\\"/\"}"
    value="${value//\\\$/\$}"
    value="${value//\\\`/\`}"
    value="${value//${placeholder}/\\}"
  elif [[ "${#value}" -ge 2 && "${value:0:1}" == "'" && "${value: -1}" == "'" ]]; then
    value="${value:1:${#value}-2}"
  fi

  printf '%s' "${value}"
}

PARSED_AMAGI_KEY=""
PARSED_AMAGI_VALUE=""
parse_amagi_env_line() {
  local line="$1"
  local raw_value

  PARSED_AMAGI_KEY=""
  PARSED_AMAGI_VALUE=""

  if [[ "${line}" =~ ^[[:space:]]*(export[[:space:]]+)?(AMAGI_[A-Z0-9_]+)[[:space:]]*=(.*)$ ]]; then
    PARSED_AMAGI_KEY="${BASH_REMATCH[2]}"
    raw_value="$(trim_leading_whitespace "${BASH_REMATCH[3]}")"
    PARSED_AMAGI_VALUE="$(decode_amagi_env_value "${raw_value}")"
    return 0
  fi

  return 1
}

collect_amagi_env_entries() {
  local env_file="$1"
  local line
  local key

  AMAGI_ENV_KEYS=()
  for key in "${!AMAGI_ENV_VALUES[@]}"; do
    unset 'AMAGI_ENV_VALUES[$key]'
  done

  [[ -n "${env_file}" && -f "${env_file}" ]] || return 0

  while IFS= read -r line || [[ -n "${line}" ]]; do
    line="${line%$'\r'}"
    if parse_amagi_env_line "${line}"; then
      if [[ -z "${AMAGI_ENV_VALUES[$PARSED_AMAGI_KEY]+x}" ]]; then
        AMAGI_ENV_KEYS+=("${PARSED_AMAGI_KEY}")
      fi
      AMAGI_ENV_VALUES["${PARSED_AMAGI_KEY}"]="${PARSED_AMAGI_VALUE}"
    fi
  done < "${env_file}"
}

escape_posix_single_quoted() {
  printf '%s' "$1" | sed "s/'/'\\\\''/g"
}

escape_fish_single_quoted() {
  printf '%s' "$1" | sed -e "s/\\\\/\\\\\\\\/g" -e "s/'/\\\\'/g"
}

write_posix_env_export_file() {
  local user_env_file="$1"
  local export_file
  local export_dir
  local temp_output
  local key
  local escaped_value

  export_file="$(resolve_posix_env_export_file)"
  export_dir="$(dirname -- "${export_file}")"
  mkdir -p "${export_dir}"
  collect_amagi_env_entries "${user_env_file}"

  if [[ "${#AMAGI_ENV_KEYS[@]}" -eq 0 ]]; then
    if [[ -f "${export_file}" ]]; then
      rm -f "${export_file}"
      printf '[amagi] removed empty POSIX env export file %s\n' "${export_file}"
    fi
    return 0
  fi

  temp_output="$(mktemp "${TMPDIR:-/tmp}/amagi-posix-env.XXXXXX")"

  {
    printf '# Generated by amagi installer. Do not edit manually.\n'
    for key in "${AMAGI_ENV_KEYS[@]}"; do
      escaped_value="$(escape_posix_single_quoted "${AMAGI_ENV_VALUES[$key]}")"
      printf "export %s='%s'\n" "${key}" "${escaped_value}"
    done
  } > "${temp_output}"

  if [[ -f "${export_file}" ]] && cmp -s "${temp_output}" "${export_file}"; then
    rm -f "${temp_output}"
    printf '[amagi] POSIX env exports already up to date in %s\n' "${export_file}"
    return 0
  fi

  mv "${temp_output}" "${export_file}"
  printf '[amagi] wrote AMAGI_* exports to %s\n' "${export_file}"
}

write_fish_env_export_file() {
  local user_env_file="$1"
  local export_file
  local export_dir
  local temp_output
  local key
  local escaped_value

  export_file="$(resolve_fish_env_export_file)"
  export_dir="$(dirname -- "${export_file}")"
  mkdir -p "${export_dir}"
  collect_amagi_env_entries "${user_env_file}"

  if [[ "${#AMAGI_ENV_KEYS[@]}" -eq 0 ]]; then
    if [[ -f "${export_file}" ]]; then
      rm -f "${export_file}"
      printf '[amagi] removed empty fish env export file %s\n' "${export_file}"
    fi
    return 0
  fi

  temp_output="$(mktemp "${TMPDIR:-/tmp}/amagi-fish-env.XXXXXX")"

  {
    printf '# Generated by amagi installer. Do not edit manually.\n'
    for key in "${AMAGI_ENV_KEYS[@]}"; do
      escaped_value="$(escape_fish_single_quoted "${AMAGI_ENV_VALUES[$key]}")"
      printf "set -gx %s -- '%s'\n" "${key}" "${escaped_value}"
    done
  } > "${temp_output}"

  if [[ -f "${export_file}" ]] && cmp -s "${temp_output}" "${export_file}"; then
    rm -f "${temp_output}"
    printf '[amagi] fish env exports already up to date in %s\n' "${export_file}"
    return 0
  fi

  mv "${temp_output}" "${export_file}"
  printf '[amagi] wrote AMAGI_* fish exports to %s\n' "${export_file}"
}

write_posix_shell_integration() {
  local install_dir="$1"
  local shell_file
  local shell_dir
  local env_export_file
  local absolute_install_dir
  local absolute_env_export_file
  local temp_output

  shell_file="$(resolve_posix_shell_file)"
  shell_dir="$(dirname -- "${shell_file}")"
  env_export_file="$(resolve_posix_env_export_file)"
  mkdir -p "${shell_dir}"

  absolute_install_dir="$(cd -- "${install_dir}" && pwd)"
  absolute_env_export_file="$(absolute_file_path "${env_export_file}")"
  temp_output="$(mktemp "${TMPDIR:-/tmp}/amagi-shell.XXXXXX")"

  {
    printf '# Generated by amagi installer. Do not edit manually.\n'
    printf "amagi_install_dir='%s'\n" "$(escape_posix_single_quoted "${absolute_install_dir}")"
    printf "amagi_path_present=0\n"
    printf 'amagi_old_ifs=$IFS\n'
    printf 'IFS=:\n'
    printf 'for amagi_path_segment in $PATH; do\n'
    printf '  if [ "$amagi_path_segment" = "$amagi_install_dir" ]; then\n'
    printf '    amagi_path_present=1\n'
    printf '    break\n'
    printf '  fi\n'
    printf 'done\n'
    printf 'IFS=$amagi_old_ifs\n'
    printf 'unset amagi_old_ifs\n'
    printf 'if [ "$amagi_path_present" -eq 0 ]; then\n'
    printf '  if [ -n "${PATH:-}" ]; then\n'
    printf '    PATH="${amagi_install_dir}:$PATH"\n'
    printf '  else\n'
    printf '    PATH="${amagi_install_dir}"\n'
    printf '  fi\n'
    printf 'fi\n'
    printf 'export PATH\n'
    printf "if [ -f '%s' ]; then\n" "$(escape_posix_single_quoted "${absolute_env_export_file}")"
    printf "  . '%s'\n" "$(escape_posix_single_quoted "${absolute_env_export_file}")"
    printf 'fi\n'
    printf 'unset amagi_install_dir\n'
    printf 'unset amagi_path_present\n'
    printf 'unset amagi_path_segment\n'
  } > "${temp_output}"

  if [[ -f "${shell_file}" ]] && cmp -s "${temp_output}" "${shell_file}"; then
    rm -f "${temp_output}"
    printf '[amagi] POSIX shell integration already up to date in %s\n' "${shell_file}"
    return 0
  fi

  mv "${temp_output}" "${shell_file}"
  printf '[amagi] wrote POSIX shell integration to %s\n' "${shell_file}"
}

write_fish_shell_integration() {
  local install_dir="$1"
  local fish_conf_file
  local fish_conf_dir
  local fish_env_export_file
  local absolute_install_dir
  local absolute_fish_env_export_file
  local temp_output

  fish_conf_file="$(resolve_fish_conf_file)"
  fish_conf_dir="$(dirname -- "${fish_conf_file}")"
  fish_env_export_file="$(resolve_fish_env_export_file)"
  mkdir -p "${fish_conf_dir}"

  absolute_install_dir="$(cd -- "${install_dir}" && pwd)"
  absolute_fish_env_export_file="$(absolute_file_path "${fish_env_export_file}")"
  temp_output="$(mktemp "${TMPDIR:-/tmp}/amagi-fish-shell.XXXXXX")"

  {
    printf '# Generated by amagi installer. Do not edit manually.\n'
    printf "set -l amagi_install_dir '%s'\n" "$(escape_fish_single_quoted "${absolute_install_dir}")"
    printf 'if not contains -- $amagi_install_dir $PATH\n'
    printf '    if set -q PATH[1]\n'
    printf '        set -gx PATH $amagi_install_dir $PATH\n'
    printf '    else\n'
    printf '        set -gx PATH $amagi_install_dir\n'
    printf '    end\n'
    printf 'end\n'
    printf "if test -f '%s'\n" "$(escape_fish_single_quoted "${absolute_fish_env_export_file}")"
    printf "    source '%s'\n" "$(escape_fish_single_quoted "${absolute_fish_env_export_file}")"
    printf 'end\n'
    printf 'set -e amagi_install_dir\n'
  } > "${temp_output}"

  if [[ -f "${fish_conf_file}" ]] && cmp -s "${temp_output}" "${fish_conf_file}"; then
    rm -f "${temp_output}"
    printf '[amagi] fish integration already up to date in %s\n' "${fish_conf_file}"
    return 0
  fi

  mv "${temp_output}" "${fish_conf_file}"
  printf '[amagi] wrote fish integration to %s\n' "${fish_conf_file}"
}

upsert_profile_source_block() {
  local profile_file="$1"
  local source_file="$2"
  local profile_dir
  local temp_output
  local line
  local in_managed_block=0
  local pending_legacy_comment=""
  local changed=0
  local escaped_source_file

  profile_dir="$(dirname -- "${profile_file}")"
  mkdir -p "${profile_dir}"
  touch "${profile_file}"
  temp_output="$(mktemp "${TMPDIR:-/tmp}/amagi-profile.XXXXXX")"
  escaped_source_file="$(escape_posix_single_quoted "${source_file}")"

  while IFS= read -r line || [[ -n "${line}" ]]; do
    line="${line%$'\r'}"

    if [[ "${in_managed_block}" -eq 1 ]]; then
      changed=1
      if [[ "${line}" == "${AMAGI_MANAGED_BLOCK_END}" ]]; then
        in_managed_block=0
      fi
      continue
    fi

    if [[ "${line}" == "${AMAGI_MANAGED_BLOCK_START}" ]]; then
      in_managed_block=1
      changed=1
      continue
    fi

    if [[ "${line}" == "${AMAGI_LEGACY_COMMENT}" ]]; then
      pending_legacy_comment="${line}"
      continue
    fi

    if [[ -n "${pending_legacy_comment}" ]]; then
      if [[ "${line}" =~ ^export[[:space:]]+PATH=\".*:\$PATH\"$ ]]; then
        pending_legacy_comment=""
        changed=1
        continue
      fi

      printf '%s\n' "${pending_legacy_comment}" >> "${temp_output}"
      pending_legacy_comment=""
    fi

    printf '%s\n' "${line}" >> "${temp_output}"
  done < "${profile_file}"

  if [[ -n "${pending_legacy_comment}" ]]; then
    printf '%s\n' "${pending_legacy_comment}" >> "${temp_output}"
  fi

  if [[ -s "${temp_output}" ]]; then
    printf '\n' >> "${temp_output}"
  fi

  {
    printf '%s\n' "${AMAGI_MANAGED_BLOCK_START}"
    printf "if [ -f '%s' ]; then\n" "${escaped_source_file}"
    printf "  . '%s'\n" "${escaped_source_file}"
    printf 'fi\n'
    printf '%s\n' "${AMAGI_MANAGED_BLOCK_END}"
  } >> "${temp_output}"

  if [[ -f "${profile_file}" ]] && cmp -s "${temp_output}" "${profile_file}"; then
    rm -f "${temp_output}"
    printf '[amagi] profile integration already up to date in %s\n' "${profile_file}"
    return 0
  fi

  mv "${temp_output}" "${profile_file}"
  if [[ "${changed}" -eq 1 ]]; then
    printf '[amagi] refreshed shell integration in %s\n' "${profile_file}"
  else
    printf '[amagi] added shell integration to %s\n' "${profile_file}"
  fi
}

print_refresh_instructions() {
  local shell_name="${SHELL##*/}"
  local refresh_file

  if [[ "${shell_name}" == "fish" ]]; then
    refresh_file="$(absolute_file_path "$(resolve_fish_conf_file)")"
  else
    refresh_file="$(absolute_file_path "$(resolve_posix_shell_file)")"
  fi

  printf '[amagi] restart your shell or run the following command to refresh PATH and AMAGI_* now:\n'
  printf '  source "%s"\n' "${refresh_file}"
}

persist_shell_integrations() {
  local install_dir="$1"
  local user_env_file
  local posix_shell_file
  local profile_file

  user_env_file="$(resolve_user_env_file)"
  write_posix_env_export_file "${user_env_file}"
  write_fish_env_export_file "${user_env_file}"
  write_posix_shell_integration "${install_dir}"
  write_fish_shell_integration "${install_dir}"

  posix_shell_file="$(absolute_file_path "$(resolve_posix_shell_file)")"
  while IFS= read -r profile_file; do
    upsert_profile_source_block "${profile_file}" "${posix_shell_file}"
  done < <(profile_files)

  if is_sourced; then
    # shellcheck source=/dev/null
    . "${posix_shell_file}"
    printf '[amagi] updated PATH and AMAGI_* in the current shell session\n'
  else
    print_refresh_instructions
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

  printf '[amagi] no local binary found, building release binary with cargo\n' >&2
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
  local platform
  local arch

  platform="$(platform_slug)"
  arch="$(arch_slug)"

  case "${platform}" in
    linux)
      printf '%s-%s-unknown-linux-musl.tar.gz\n' "${BIN_NAME}" "${arch}"
      ;;
    macos)
      printf '%s-%s-apple-darwin.tar.gz\n' "${BIN_NAME}" "${arch}"
      ;;
    *)
      printf '[amagi] unsupported platform for remote install asset naming: %s\n' "${platform}" >&2
      return 1
      ;;
  esac
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
    printf '%shttps://github.com/%s/%s/releases/latest/download/%s\n' \
      "${PROXY_PREFIX}" \
      "${REMOTE_REPO_OWNER}" "${REMOTE_REPO_NAME}" "${asset_name}"
  else
    printf '%shttps://github.com/%s/%s/releases/download/%s/%s\n' \
      "${PROXY_PREFIX}" \
      "${REMOTE_REPO_OWNER}" "${REMOTE_REPO_NAME}" "${REMOTE_VERSION}" "${asset_name}"
  fi
}

download_remote_binary() {
  local url
  local download_path
  local asset_name

  asset_name="$(remote_asset_name)" || return 1
  url="$(remote_download_url)" || return 1
  download_path="$(mktemp "${TMPDIR:-/tmp}/amagi-install.XXXXXX.${asset_name##*.}")"
  REMOTE_DOWNLOAD_PATH="${download_path}"

  printf '[amagi] downloading %s\n' "${url}" >&2

  if command -v curl >/dev/null 2>&1; then
    curl -fsSL "${url}" -o "${download_path}"
  elif command -v wget >/dev/null 2>&1; then
    wget -qO "${download_path}" "${url}"
  else
    printf '[amagi] curl or wget is required for remote install.\n' >&2
    rm -f "${download_path}"
    return 1
  fi

  printf '%s\n' "${download_path}"
}

extract_remote_binary() {
  local archive_path="$1"
  local asset_name
  local extracted_binary

  asset_name="$(remote_asset_name)" || return 1
  REMOTE_EXTRACT_DIR="$(mktemp -d "${TMPDIR:-/tmp}/amagi-extract.XXXXXX")"

  case "${asset_name}" in
    *.tar.gz)
      if ! command -v tar >/dev/null 2>&1; then
        printf '[amagi] tar is required for remote install.\n' >&2
        return 1
      fi
      tar -xzf "${archive_path}" -C "${REMOTE_EXTRACT_DIR}"
      ;;
    *)
      printf '[amagi] unsupported remote asset format: %s\n' "${asset_name}" >&2
      return 1
      ;;
  esac

  extracted_binary="${REMOTE_EXTRACT_DIR}/${BIN_NAME}"
  if [[ ! -f "${extracted_binary}" ]]; then
    printf '[amagi] extracted archive did not contain %s\n' "${BIN_NAME}" >&2
    return 1
  fi

  chmod 755 "${extracted_binary}"
  printf '%s\n' "${extracted_binary}"
}

cleanup_remote_temp() {
  if [[ -n "${REMOTE_DOWNLOAD_PATH}" && -f "${REMOTE_DOWNLOAD_PATH}" ]]; then
    rm -f "${REMOTE_DOWNLOAD_PATH}"
  fi

  if [[ -n "${REMOTE_EXTRACT_DIR}" && -d "${REMOTE_EXTRACT_DIR}" ]]; then
    rm -rf "${REMOTE_EXTRACT_DIR}"
  fi
}

if [[ -z "${INSTALL_DIR}" ]]; then
  INSTALL_DIR="$(default_install_dir)"
fi
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

  SOURCE_BINARY="$(extract_remote_binary "${SOURCE_BINARY}" || true)"
  if [[ -z "${SOURCE_BINARY}" ]]; then
    cleanup_remote_temp
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
  cleanup_remote_temp
fi

printf '[amagi] installed to %s\n' "${INSTALL_PATH}"
sync_user_env_file "$(resolve_project_env_source || true)"
persist_shell_integrations "${INSTALL_DIR}"
