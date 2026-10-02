---
name: bash-clean-code
description: >-
  Enforces production standards for Bash and POSIX shell scripting. Mandates strict error handling
  (set -euo pipefail), zero ShellCheck warnings, temporary file cleanup traps, XDG directory compliance,
  safe quoting, portable architecture/OS detection, and modular function limits.
---

# `bash-clean-code` Skill: Robust & Scalable Shell Scripting

Enforces strict production standards for shell scripts, CLI installers, and system automation. Scripts must be deterministic, portable, safely quoted, and free of ShellCheck warnings.

---

## 1. Mandatory Script Preamble

Every bash script must begin with a strict execution header:

```bash
#!/usr/bin/env bash
set -euo pipefail
IFS=$'\n\t'
```

### Why:
- `-e`: Exit immediately on command failure.
- `-u`: Treat unset variables as an error and exit immediately.
- `-o pipefail`: Return the exit status of the last failed command in a pipeline.
- `IFS=$'\n\t'`: Prevent word splitting on spaces.

---

## 2. Zero Warnings & Linter Policy

1. **Zero ShellCheck Warnings**:
   - Every script must pass without warnings:
     ```bash
     shellcheck -x script.sh
     ```
   - **Never** use `# shellcheck disable=...` to mask unhandled errors or sloppy quoting. Fix the root cause.
2. **Strict Quoting Discipline**:
   - Always double-quote variables and command substitutions: `"$var"`, `"$(command)"`.
   - Never leave unquoted paths or user inputs.

---

## 3. Safe Temporary Files & Exit Traps

**Never** leave temporary files or orphaned resources on disk. Always use a dedicated temporary directory with an automated cleanup trap:

```bash
TMP_DIR="$(mktemp -d -t kbuild_tmp.XXXXXXXXXX)"
cleanup() {
    local exit_code=$?
    rm -rf "$TMP_DIR"
    exit "$exit_code"
}
trap cleanup EXIT INT TERM HUP
```

---

## 4. XDG Base Directory Compliance

**Never** pollute `$HOME` directly with dotfiles, state, or cache files. Always respect XDG specifications:

```bash
XDG_CONFIG_HOME="${XDG_CONFIG_HOME:-$HOME/.config}"
XDG_DATA_HOME="${XDG_DATA_HOME:-$HOME/.local/share}"
XDG_STATE_HOME="${XDG_STATE_HOME:-$HOME/.local/state}"
XDG_CACHE_HOME="${XDG_CACHE_HOME:-$HOME/.cache}"
```

---

## 5. Portable OS & Architecture Detection

When building installers or system scripts, detect architectures and operating systems deterministically:

```bash
detect_target() {
    local os arch
    os="$(uname -s | tr '[:upper:]' '[:lower:]')"
    arch="$(uname -m)"

    case "$arch" in
        x86_64|amd64) TARGET_ARCH="x86_64" ;;
        aarch64|arm64) TARGET_ARCH="aarch64" ;;
        *) echo "Error: Unsupported architecture: $arch" >&2; return 1 ;;
    esac

    case "$os" in
        linux) TARGET_OS="unknown-linux-gnu" ;;
        darwin) TARGET_OS="apple-darwin" ;;
        *) echo "Error: Unsupported OS: $os" >&2; return 1 ;;
    esac

    TARGET="${TARGET_ARCH}-${TARGET_OS}"
}
```

---

## 6. Quantitative Limits & Modularity

| Metric | Target | Hard Limit | Action if Exceeded |
| :--- | :--- | :--- | :--- |
| **File Length** | 100–150 lines | **250 lines** | Split into modular sub-scripts or lib files |
| **Function Length** | 15–25 lines | **40 lines** | Decompose into focused single-purpose helpers |
| **Variables Scope** | Local by default | **Avoid Globals** | Declare local variables with `local var="..."` |
| **Exit Codes** | Semantic | **0, 1, 130** | 0 = Success, 1 = Error, 130 = Interrupted (SIGINT) |

---

## 7. Logging & Terminal Output Standards

1. **Error Output to Stderr**:
   ```bash
   log_info()  { printf "\033[1;34mℹ\033[0m %s\n" "$*" >&2; }
   log_warn()  { printf "\033[1;33m⚠\033[0m %s\n" "$*" >&2; }
   log_error() { printf "\033[1;31m✖\033[0m %s\n" "$*" >&2; }
   log_ok()    { printf "\033[1;32m✔\033[0m %s\n" "$*" >&2; }
   ```
2. **TTY Awareness**:
   Only print colors and interactive prompts when attached to a real terminal:
   ```bash
   if [ -t 1 ]; then
       COLOR_BOLD="\033[1m"
       COLOR_RESET="\033[0m"
   else
       COLOR_BOLD=""
       COLOR_RESET=""
   fi
   ```

---

## 8. Verification Checklist

Before finishing changes to any shell script:

```bash
# 1. Syntax check
bash -n script.sh

# 2. Strict static analysis
shellcheck -x script.sh

# 3. Dry run execution
./script.sh --dry-run
```
