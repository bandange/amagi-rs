#!/usr/bin/env bash
set -euo pipefail

BIN_NAME="amagi"
INSTALL_DIR="${AMAGI_INSTALL_DIR:-}"
HAS_EXPLICIT_INSTALL_DIR=0
KEEP_PATH=0
KEEP_USER_ENV=0
ASSUME_YES=0
SCRIPT_PATH="${BASH_SOURCE[0]:-}"
SCRIPT_DIR=""
REPO_ROOT=""
PATH_CLEANED=0
USER_ENV_CLEANED=0
CURRENT_PATH_CLEANED=0
AMAGI_MANAGED_BLOCK_START="# >>> amagi installer >>>"
AMAGI_MANAGED_BLOCK_END="# <<< amagi installer <<<"
AMAGI_LEGACY_COMMENT="# amagi installer"

if [[ -n "${SCRIPT_PATH}" ]]; then
  SCRIPT_DIR="$(cd -- "$(dirname -- "${SCRIPT_PATH}")" && pwd)"
  REPO_ROOT="$(cd -- "${SCRIPT_DIR}/.." 2>/dev/null && pwd || true)"
fi

if [[ -n "${INSTALL_DIR}" ]]; then
  HAS_EXPLICIT_INSTALL_DIR=1
fi

while [[ $# -gt 0 ]]; do
  case "$1" in
    --install-dir)
      INSTALL_DIR="$2"
      HAS_EXPLICIT_INSTALL_DIR=1
      shift 2
      ;;
    --keep-path)
      KEEP_PATH=1
      shift
      ;;
    --keep-user-env)
      KEEP_USER_ENV=1
      shift
      ;;
    --yes)
      ASSUME_YES=1
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

normalize_dir() {
  local value="$1"

  [[ -n "${value}" ]] || return 1

  value="${value%/}"
  [[ -n "${value}" ]] || value="/"

  if [[ -d "${value}" ]]; then
    (
      cd -- "${value}"
      pwd
    )
  else
    printf '%s\n' "${value}"
  fi
}

add_unique_dir() {
  local value="$1"
  local normalized
  local existing

  normalized="$(normalize_dir "${value}" 2>/dev/null || true)"
  [[ -n "${normalized}" ]] || return 0
  if [[ "${HAS_EXPLICIT_INSTALL_DIR}" -eq 0 && "$(should_skip_auto_dir "${normalized}")" == "yes" ]]; then
    return 0
  fi

  for existing in "${CANDIDATE_DIRS[@]}"; do
    if [[ "${existing}" == "${normalized}" ]]; then
      return 0
    fi
  done

  CANDIDATE_DIRS+=("${normalized}")
}

dirs_from_path_env() {
  local path_value="$1"
  local entry
  local entries

  [[ -n "${path_value}" ]] || return 0

  IFS=':' read -r -a entries <<< "${path_value}"
  for entry in "${entries[@]}"; do
    [[ -n "${entry}" ]] || continue
    if [[ -f "${entry}/${BIN_NAME}" ]]; then
      add_unique_dir "${entry}"
    fi
  done
}

should_skip_auto_dir() {
  local dir="$1"
  local cargo_dir

  cargo_dir="$(normalize_dir "${HOME}/.cargo/bin" 2>/dev/null || true)"
  if [[ -n "${cargo_dir}" && "${dir}" == "${cargo_dir}" ]]; then
    printf 'yes\n'
    return 0
  fi

  printf 'no\n'
}

is_owned_install_dir() {
  local dir="$1"
  local parent

  parent="$(dirname -- "${dir}")"

  if [[ "$(basename -- "${dir}")" == "amagi" ]]; then
    return 0
  fi

  if [[ "$(basename -- "${dir}")" == "bin" && "$(basename -- "${parent}")" == "amagi" ]]; then
    return 0
  fi

  return 1
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

confirm_environment_cleanup() {
  local prompt="$1"
  local skip_label="$2"
  local reply

  if [[ "${ASSUME_YES}" -eq 1 ]]; then
    return 0
  fi

  if [[ ! -t 0 ]]; then
    printf '[amagi] skipped %s because confirmation requires an interactive terminal; rerun with --yes to allow it\n' "${skip_label}"
    return 1
  fi

  printf '[amagi] %s [y/N] ' "${prompt}"
  read -r reply
  case "${reply}" in
    [Yy]|[Yy][Ee][Ss])
      return 0
      ;;
    *)
      printf '[amagi] skipped %s\n' "${skip_label}"
      return 1
      ;;
  esac
}

profile_contains_path_entry() {
  local file="$1"
  local entry="$2"
  local target_line
  local line

  [[ -f "${file}" ]] || return 1

  target_line="export PATH=\"${entry}:\$PATH\""

  while IFS= read -r line || [[ -n "${line}" ]]; do
    line="${line%$'\r'}"
    if [[ "${line}" == "${target_line}" ]]; then
      return 0
    fi
  done < "${file}"

  return 1
}

profile_contains_managed_block() {
  local file="$1"

  [[ -f "${file}" ]] || return 1
  grep -Fq "${AMAGI_MANAGED_BLOCK_START}" "${file}"
}

has_path_cleanup_targets() {
  local candidate_dir
  local profile_file

  for candidate_dir in "${CANDIDATE_DIRS[@]}"; do
    if [[ ":${PATH}:" == *":${candidate_dir}:"* ]]; then
      return 0
    fi

    while IFS= read -r profile_file; do
      if profile_contains_path_entry "${profile_file}" "${candidate_dir}"; then
        return 0
      fi
    done < <(profile_files)
  done

  while IFS= read -r profile_file; do
    if profile_contains_managed_block "${profile_file}"; then
      return 0
    fi
  done < <(profile_files)

  if [[ -f "$(resolve_posix_shell_file)" ]]; then
    return 0
  fi

  if [[ -f "$(resolve_posix_env_export_file)" ]]; then
    return 0
  fi

  if [[ -f "$(resolve_fish_conf_file)" ]]; then
    return 0
  fi

  if [[ -f "$(resolve_fish_env_export_file)" ]]; then
    return 0
  fi

  return 1
}

has_user_env_entries() {
  local user_env_file
  local line

  user_env_file="$(resolve_user_env_file)"
  [[ -f "${user_env_file}" ]] || return 1

  while IFS= read -r line || [[ -n "${line}" ]]; do
    line="${line%$'\r'}"
    if [[ "${line}" =~ ^[[:space:]]*(export[[:space:]]+)?AMAGI_[A-Z0-9_]+[[:space:]]*= ]]; then
      return 0
    fi
  done < "${user_env_file}"

  return 1
}

has_env_export_artifacts() {
  [[ -f "$(resolve_posix_env_export_file)" ]] && return 0
  [[ -f "$(resolve_fish_env_export_file)" ]] && return 0
  return 1
}

remove_path_entry_from_file() {
  local file="$1"
  local entry="$2"
  local temp_file
  local target_line
  local in_managed_block=0
  local pending_comment=""
  local changed=0
  local line

  [[ -f "${file}" ]] || return 0

  temp_file="$(mktemp "${TMPDIR:-/tmp}/amagi-uninstall.XXXXXX")"
  target_line="export PATH=\"${entry}:\$PATH\""

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
      pending_comment="${line}"
      continue
    fi

    if [[ "${line}" == "${target_line}" ]]; then
      pending_comment=""
      changed=1
      continue
    fi

    if [[ -n "${pending_comment}" ]]; then
      printf '%s\n' "${pending_comment}" >> "${temp_file}"
      pending_comment=""
    fi

    printf '%s\n' "${line}" >> "${temp_file}"
  done < "${file}"

  if [[ -n "${pending_comment}" ]]; then
    printf '%s\n' "${pending_comment}" >> "${temp_file}"
  fi

  if [[ "${changed}" -eq 1 ]]; then
    mv "${temp_file}" "${file}"
    PATH_CLEANED=1
    printf '[amagi] removed PATH entry from %s\n' "${file}"
  else
    rm -f "${temp_file}"
  fi
}

remove_file_if_present() {
  local file="$1"
  local label="$2"

  if [[ ! -f "${file}" ]]; then
    return 1
  fi

  rm -f "${file}"
  printf '[amagi] removed %s %s\n' "${label}" "${file}"
  return 0
}

remove_shell_integration_artifacts() {
  local removed=0
  local shell_integration_dir

  if remove_file_if_present "$(resolve_posix_shell_file)" "POSIX shell integration"; then
    removed=1
  fi

  if remove_file_if_present "$(resolve_posix_env_export_file)" "POSIX env export file"; then
    removed=1
  fi

  if remove_file_if_present "$(resolve_fish_conf_file)" "fish integration"; then
    removed=1
  fi

  if remove_file_if_present "$(resolve_fish_env_export_file)" "fish env export file"; then
    removed=1
  fi

  shell_integration_dir="$(resolve_shell_integration_dir)"
  remove_empty_amagi_dir "${shell_integration_dir}"

  if [[ "${removed}" -eq 1 ]]; then
    PATH_CLEANED=1
  fi
}

remove_env_export_artifacts() {
  local removed=0
  local shell_integration_dir

  if remove_file_if_present "$(resolve_posix_env_export_file)" "POSIX env export file"; then
    removed=1
  fi

  if remove_file_if_present "$(resolve_fish_env_export_file)" "fish env export file"; then
    removed=1
  fi

  shell_integration_dir="$(resolve_shell_integration_dir)"
  remove_empty_amagi_dir "${shell_integration_dir}"

  if [[ "${removed}" -eq 1 ]]; then
    USER_ENV_CLEANED=1
  fi
}

remove_binary_from_dir() {
  local dir="$1"
  local binary_path="${dir}/${BIN_NAME}"

  if [[ ! -e "${binary_path}" ]]; then
    return 1
  fi

  rm -f "${binary_path}"
  printf '[amagi] removed %s\n' "${binary_path}"
  return 0
}

remove_empty_dir_if_owned() {
  local dir="$1"
  local parent

  [[ -d "${dir}" ]] || return 0
  is_owned_install_dir "${dir}" || return 0

  if find "${dir}" -mindepth 1 -print -quit 2>/dev/null | grep -q .; then
    return 0
  fi

  rmdir "${dir}" 2>/dev/null || return 0
  printf '[amagi] removed empty directory %s\n' "${dir}"

  parent="$(dirname -- "${dir}")"
  if [[ "$(basename -- "${dir}")" == "bin" && "$(basename -- "${parent}")" == "amagi" ]]; then
    if [[ -d "${parent}" ]] && ! find "${parent}" -mindepth 1 -print -quit 2>/dev/null | grep -q .; then
      rmdir "${parent}" 2>/dev/null || true
      if [[ ! -d "${parent}" ]]; then
        printf '[amagi] removed empty directory %s\n' "${parent}"
      fi
    fi
  fi
}

remove_empty_amagi_dir() {
  local dir="$1"

  [[ -d "${dir}" ]] || return 0
  [[ "$(basename -- "${dir}")" == "amagi" ]] || return 0

  if find "${dir}" -mindepth 1 -print -quit 2>/dev/null | grep -q .; then
    return 0
  fi

  rmdir "${dir}" 2>/dev/null || return 0
  printf '[amagi] removed empty directory %s\n' "${dir}"
}

remove_user_env_entries() {
  local user_env_file
  local user_env_dir
  local temp_file
  local changed=0
  local line

  user_env_file="$(resolve_user_env_file)"
  [[ -f "${user_env_file}" ]] || return 0

  temp_file="$(mktemp "${TMPDIR:-/tmp}/amagi-user-env.XXXXXX")"

  while IFS= read -r line || [[ -n "${line}" ]]; do
    line="${line%$'\r'}"

    if [[ "${line}" =~ ^[[:space:]]*(export[[:space:]]+)?AMAGI_[A-Z0-9_]+[[:space:]]*= ]]; then
      changed=1
      continue
    fi

    printf '%s\n' "${line}" >> "${temp_file}"
  done < "${user_env_file}"

  if [[ "${changed}" -eq 1 ]]; then
    USER_ENV_CLEANED=1
    if grep -q '[^[:space:]]' "${temp_file}"; then
      mv "${temp_file}" "${user_env_file}"
      printf '[amagi] removed AMAGI_* entries from %s\n' "${user_env_file}"
    else
      rm -f "${temp_file}"
      rm -f "${user_env_file}"
      printf '[amagi] removed empty user env file %s\n' "${user_env_file}"
    fi

    user_env_dir="$(dirname -- "${user_env_file}")"
    remove_empty_amagi_dir "${user_env_dir}"
  else
    rm -f "${temp_file}"
  fi
}

update_current_shell_path() {
  local filtered=()
  local entry
  local keep
  local candidate
  local entries

  IFS=':' read -r -a entries <<< "${PATH}"
  for entry in "${entries[@]}"; do
    keep=1

    for candidate in "${CANDIDATE_DIRS[@]}"; do
      if [[ "${entry%/}" == "${candidate%/}" ]]; then
        keep=0
        CURRENT_PATH_CLEANED=1
        break
      fi
    done

    if [[ "${keep}" -eq 1 ]]; then
      filtered+=("${entry}")
    fi
  done

  PATH="$(IFS=:; printf '%s' "${filtered[*]}")"
  export PATH
}

CANDIDATE_DIRS=()

if [[ -n "${INSTALL_DIR}" ]]; then
  add_unique_dir "${INSTALL_DIR}"
fi

if [[ "${HAS_EXPLICIT_INSTALL_DIR}" -eq 0 ]]; then
  add_unique_dir "$(default_install_dir)"

  if command -v "${BIN_NAME}" >/dev/null 2>&1; then
    resolved_command="$(command -v "${BIN_NAME}" || true)"
    if [[ -n "${resolved_command}" && -f "${resolved_command}" ]]; then
      add_unique_dir "$(dirname -- "${resolved_command}")"
    fi
  fi

  dirs_from_path_env "${PATH}"
fi

found_any=0
for candidate_dir in "${CANDIDATE_DIRS[@]}"; do
  if remove_binary_from_dir "${candidate_dir}"; then
    found_any=1
  fi

  remove_empty_dir_if_owned "${candidate_dir}"
done

if [[ "${KEEP_PATH}" -eq 0 && "${#CANDIDATE_DIRS[@]}" -gt 0 ]]; then
  if has_path_cleanup_targets; then
    if confirm_environment_cleanup \
      "remove matching shell profile entries, helper files, and the current shell PATH?" \
      "PATH cleanup"; then
      PATH_CLEANED=1
      for candidate_dir in "${CANDIDATE_DIRS[@]}"; do
        while IFS= read -r profile_file; do
          remove_path_entry_from_file "${profile_file}" "${candidate_dir}"
        done < <(profile_files)
      done
      remove_shell_integration_artifacts
      update_current_shell_path
      printf '[amagi] removed matching PATH entries where present\n'
    fi
  fi
fi

if [[ "${KEEP_USER_ENV}" -eq 0 ]]; then
  if has_user_env_entries || has_env_export_artifacts; then
    user_env_file="$(resolve_user_env_file)"
    if confirm_environment_cleanup \
      "remove AMAGI_* entries and exported shell env files from ${user_env_file}?" \
      "user env cleanup"; then
      if has_user_env_entries; then
        remove_user_env_entries
      fi
      remove_env_export_artifacts
    fi
  fi
fi

if [[ "${found_any}" -eq 0 && "${PATH_CLEANED}" -eq 0 && "${USER_ENV_CLEANED}" -eq 0 && "${CURRENT_PATH_CLEANED}" -eq 0 ]]; then
  printf '[amagi] no installed binary or persisted configuration found.\n'
else
  printf '[amagi] uninstall complete\n'
fi
