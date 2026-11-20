---
name: rbuild
description: Offloads Rust crate and workspace compilation and verification (check/clippy/test) to remote GitHub Codespaces runners with zero local CPU overhead, persistent SSD caching, and automatic binary retrieval.
---

# ⚡ rbuild — Remote Rust Compiler

Use `rbuild` whenever you need to compile a Rust project, crate, or binary without consuming local CPU, RAM, or battery. It mirrors the ergonomics of `aur-builder` / `abuild`, streaming local source code over an encrypted SSH pipe into a dedicated GitHub Codespace (4 vCPUs, 16GB RAM) backed by persistent SSD caching (`mold` linker + `sccache` + Cargo cache), then downloading the compiled binary directly into your local workspace or `~/.local/bin/`.

---

## 🚀 When to Use

- **Heavy Crates**: Compiling projects with large dependency trees (Tokio, Axum, Bevy, Serde, Syn, Wasm, etc.) where local compilation takes several minutes or pegs all CPU cores.
- **Resource Conservation**: Compiling code without maxing local CPU cores or causing fan noise and thermal throttling.
- **Incremental Fast Builds**: Making iterative changes to code while taking advantage of persistent remote `target/` directories and `sccache` hits.
- **Autonomous Agent Workflows**: Any AI agent tasked with building or verifying a Rust project can run `rbuild` directly.
- **Remote Verification**: Run `cargo check`, `cargo clippy`, or `cargo test` on the remote runner with `rbuild --check`, `--clippy`, or `--test` (no binary download).

### ⚖️ Local vs Remote Decision Rule (Quota Awareness)
- **Do It Local If**:
  - The monthly or weekly free tier Codespaces duration is running low (<5–10 hours left).
  - The project is small, simple, or has minimal dependencies (where local compile finishes in a few seconds).
- **Use `rbuild` If**:
  - Healthy free tier hours remain on the account.
  - Compiling large workspaces or crates with heavy dependencies that take minutes.
  - Doing heavy release builds with Link Time Optimization (LTO).
  - You want to keep the local machine completely quiet with cold CPUs.

---

## 🛠️ Usage & Commands

### 1. Build Current Local Project (Release Mode)
```bash
# Run inside any directory containing a Cargo.toml
rbuild

# Output binary lands in:
# ./target/remote/release/<binary_name>
```

### 2. Build Specific Binary Target
```bash
rbuild --bin my-tool
```

### 3. Build Specific Workspace Member
```bash
rbuild -p my-subcrate
```

### 4. Build and Install Directly to `~/.local/bin/`
```bash
rbuild -i
# or
rbuild --install
```

### 5. Debug Build (instead of release)
```bash
rbuild --debug
```

### 6. Clean or Force Rebuild
```bash
# Clean remote target for this project before building
rbuild --clean

# Force rebuild without skipping
rbuild -f
```

### 7. Custom Target Triple (Cross-Compilation)
```bash
rbuild --target x86_64-unknown-linux-musl
```

### 8. Keep Runner Warm for Consecutive Iterations
```bash
# By default, rbuild automatically stops the Codespace after retrieval to save quota.
# Pass -w or --keep-warm during rapid debugging to leave the VM running:
rbuild -w
```

### 9. Codespace Management & Maintenance
```bash
# Check status of builder Codespaces and storage
rbuild status

# Stop builder Codespace immediately to conserve monthly quota
rbuild stop

# Open an interactive SSH shell into the remote runner
rbuild ssh

# Purge all remote project targets to reclaim SSD space
rbuild clean
```

### 10. Remote Verification (no binary download)
```bash
# Run `cargo check --all-targets` on the remote runner
rbuild --check

# Run `cargo clippy --all-targets -- -D warnings` on the remote runner
rbuild --clippy

# Run `cargo test --all-targets` on the remote runner
rbuild --test
```

---

## 🧠 How It Works Under the Hood

```text
┌─────────────────────────────────┐
│  💻 Local Machine               │
│  $ rbuild                       │
└──────────────┬──────────────────┘
               │  1. Discover Cargo.toml & compute source hash
               │  2. Stream source tarball over SSH (skipping local target/ & .git/)
               ▼
┌────────────────────────────────────────────────────────┐
│  ☁️ Dedicated GitHub Codespace (4 vCPUs, 16GB RAM)     │
│  • mold linker for sub-second linking                  │
│  • sccache compiler cache                              │
│  • Persistent SSD storage (incremental target/ kept)   │
│  • Auto-purges oldest targets when storage > 80%       │
└──────────────┬─────────────────────────────────────────┘
               │  3. Compiles via 'cargo build --release'
               │  4. Streams binary back over SSH
               ▼
┌─────────────────────────────────┐
│  📦 target/remote/release/<bin> │
│  (or ~/.local/bin/<bin>)        │
└─────────────────────────────────┘
```

---

## ⏱️ Performance Profile

- **First Run (Cold Dependencies)**: Codespace downloads dependencies once and caches them to SSD.
- **Subsequent Runs (Warm Cache)**: Only your modified `.rs` files are recompiled. Linking finishes in under 1 second via `mold`. Typical incremental build time: **2–4 seconds**.
- **Storage Protection**: If Codespace disk usage exceeds 80%, `rbuild` automatically purges the oldest inactive `target/` directories before compiling.
- **Auto-Stop By Default**: The Codespace runner is immediately stopped after streaming the binary to disk to prevent wasting monthly free tier hours. If you are doing fast back-to-back iterations, pass `-w` (`--keep-warm`).
