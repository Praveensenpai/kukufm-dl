---
name: git-release-craft
description: >-
  Automates the complete Git release lifecycle: atomic conventional commits, semantic version
  bumps, mandatory Linux x86_64 GitHub Actions workflows for binary apps, rich aesthetic release
  descriptions, tag creation, and autonomous closed-loop workflow verification and self-healing.
---

# `git-release-craft` Skill: Release Lifecycle & Workflow Verification

Automates production releases with aesthetic, well-formatted release notes, mandatory multi-arch GitHub Actions workflows for binary applications, and autonomous verification and self-healing of CI/CD pipelines.

---

## 1. The Golden Rules: Never Stop at Git Push & Never Fire-and-Forget

1. **Mandatory Binary Release Trigger (Never Stop at `git push`)**:
   For any repository producing or distributing standalone compiled binaries (Rust CLI, Go, C/C++, etc.) or embedding assets at compile time (such as `karakuri`), **a code modification is NEVER complete at `git push`**. Pushing to `main` without releasing leaves binary users and installer scripts with stale builds. Every commit modifying code, dependencies, or compile-time embedded assets MUST automatically trigger the complete release lifecycle: version bump, release tag, aesthetic notes, and closed-loop CI verification. Never wait for the user to ask "why didn't you release a tag?".
2. **Ensure Release Workflows Exist**: Any project producing standalone compiled binaries must have a verified `.github/workflows/release.yml`.
3. **Craft a Proper Description**: Never use blank releases or bare `--generate-notes`. Every release must follow the aesthetic highlight format.
4. **Actively Monitor Workflows**: Watch the triggered GitHub Actions CI/Release pipelines until completion.
5. **Autonomously Heal Failures**: If GitHub Actions or release builds fail, diagnose and fix them immediately without asking for permission to resolve release errors.
6. **Verify Release Assets**: Confirm that compiled binaries, packages, or checksums are physically generated and attached to the release.

---

## 2. Mandatory Release Workflow for Binary Projects (Linux x86_64)

Before publishing a release for any repository that produces a standalone binary (e.g. Rust CLI or application, Go, C/C++):

1. **Verify Workflow Existence**:
   Check if `.github/workflows/release.yml` exists. If missing, create it before tagging. Releases are strictly targeted to **Linux `x86_64` (`x86_64-unknown-linux-gnu`)** on `ubuntu-latest`.

2. **Standard Linux x86_64 Release Workflow Template (Rust Example)**:
   Place at `.github/workflows/release.yml`:

```yaml
name: Release

on:
  push:
    tags:
      - 'v*'

permissions:
  contents: write

jobs:
  publish:
    name: Build & Publish (x86_64-unknown-linux-gnu)
    runs-on: ubuntu-latest

    steps:
      - name: Checkout repository
        uses: actions/checkout@v4

      - name: Install Rust toolchain
        uses: dtolnay/rust-toolchain@stable
        with:
          targets: x86_64-unknown-linux-gnu

      - name: Setup Rust Cache
        uses: Swatinem/rust-cache@v2
        with:
          key: x86_64-unknown-linux-gnu
          cache-all-crates: "true"

      - name: Build release binary
        run: cargo build --release --target x86_64-unknown-linux-gnu

      - name: Package archive & compute checksums
        shell: bash
        run: |
          mkdir -p dist
          SRC_BIN="target/x86_64-unknown-linux-gnu/release/<binary_name>"
          cp "${SRC_BIN}" <binary_name>
          tar -czf "dist/<binary_name>-x86_64-linux.tar.gz" <binary_name>
          cd dist
          sha256sum * > "<binary_name>-x86_64-linux.tar.gz.sha256"

      - name: Upload archives to GitHub Release
        uses: softprops/action-gh-release@v2
        with:
          files: dist/*
```

---

## 3. Pre-Release Quality Checklist

Before tagging or creating a release:

1. **Verify Local Quality**:
   - Rust: `cargo test --all-targets && cargo clippy --all-targets -- -D warnings && cargo fmt --check`
   - Python: `uv run pytest && uv run ruff check && uv run ruff format --check && uv run mypy .`
2. **Bump Semantic Version**:
   - Update `Cargo.toml`, `pyproject.toml`, or `package.json` to target version `vX.Y.Z`.
3. **Commit Manifest Changes**:
   ```bash
   git add Cargo.toml Cargo.lock  # or pyproject.toml / uv.lock
   git commit -m "chore(release): bump to vX.Y.Z"
   ```

---

## 4. Aesthetic Release Note Template (Mandatory)

Every release must have a rich, beautifully structured description matching this format:

```markdown
# 🌸 <Project Name> vX.Y.Z ✨

<One-sentence punchy summary highlighting the theme or milestone of this release>

### 🌟 Key Highlights (or 🌸 What's New in vX.Y.Z)

- **• <Icon> <Feature / Fix Name>**: <Clear, concrete explanation of what changed and its user impact>.
- **• <Icon> <Feature / Fix Name>**: <Clear, concrete explanation of what changed and its user impact>.
- **• <Icon> <Feature / Fix Name>**: <Clear, concrete explanation of what changed and its user impact>.
- **• ✒️ Strict Codebase Quality**: <Mention code quality achievements: 100% tests passing, zero warnings, clean architecture>.

### 🚀 Direct Download & Run

```bash
<One-line curl installer, cargo install, or package manager command>
```
```

### Example Icons to Use:
- `✨` / `🌟` : Major new features
- `⚡` / `🚀` : Performance boosts, cloud offloading
- `📦` : Standalone binaries, asset bundling
- `🤫` / `🛡️` : Silent clutter-free logging, security, safety
- `🌸` / `🎨` : UI/TUI polish, aesthetics, styling
- `🐛` / `🔧` : Bug fixes, stability patches

---

## 5. Tagging & Publishing the Release

```bash
# 1. Create annotated tag locally
git tag -a vX.Y.Z -m "<Project Name> vX.Y.Z: <Short Summary>"

# 2. Push commit and tag to remote
git push origin <branch>
git push origin vX.Y.Z

# 3. Write release notes to a temporary file
cat << 'NOTES_EOF' > /tmp/release_notes.md
<Aesthetic Release Notes from Section 4>
NOTES_EOF

# 4. Create the official GitHub release
gh release create vX.Y.Z \
  --title "🌸 <Project Name> vX.Y.Z ✨" \
  --notes-file /tmp/release_notes.md

rm -f /tmp/release_notes.md
```

---

## 6. Post-Release Verification & Autonomous Self-Healing

Immediately after publishing the release:

### Step 6.1: Retrieve Triggered Actions
```bash
gh run list --limit 4 --json databaseId,name,status,conclusion,headBranch,url
```

### Step 6.2: Watch and Verify Workflows
Wait for all active workflows (`Release`, `CI`, `Build`) to finish:
```bash
gh run watch <run_id>
```

### Step 6.3: Autonomous Failure Recovery Loop (Zero Permission-Seeking)
If any workflow fails (`conclusion: failure`):

> [!IMPORTANT]
> **Do NOT ask the user for permission to fix a failed release workflow!**
> Never ask: *"I made a mistake, would you like me to fix it?"* or *"Do you allow me to fix this?"*.
> The error happened during the release execution initiated by the agent. You are 100% responsible for delivering a working, green release. Fix it autonomously.

**Autonomous Healing Protocol**:
1. **Inspect Failed Logs**:
   ```bash
   gh run view <run_id> --log-failed
   ```
2. **Diagnose Root Cause**:
   - Compiler or clippy errors on matrix targets.
   - Missing cross-compilation linker (e.g. `aarch64-linux-gnu-gcc`).
   - Packaging path or binary naming mismatch.
   - GitHub Actions permission issues (`contents: write`).
3. **Fix Immediately**:
   Apply code or workflow fixes directly in the workspace.
4. **Verify Locally**:
   Run tests and lints locally (`cargo test`, `cargo clippy`, `cargo fmt`).
5. **Update Release Tag & Re-Push**:
   ```bash
   git add -A
   git commit -m "fix(ci): resolve release build failure for <target>"
   git push origin <branch>
   git tag -fa vX.Y.Z -m "<Project Name> vX.Y.Z: <Summary>"
   git push origin vX.Y.Z --force
   ```
6. **Re-watch & Iterate**:
   Monitor the newly triggered workflow run until all jobs are **GREEN**. Repeat until successful.

### Step 6.4: Inspect Uploaded Assets
Confirm that release artifacts are physically attached to the release:
```bash
gh release view vX.Y.Z
```
Verify:
- Pre-compiled binaries/archives exist (e.g. `.tar.gz`, `.zip`, `.whl`).
- Cross-platform targets are present (`x86_64` and `aarch64`).

---

## 7. Stale Tag & Release Cleanup (Hygiene)

When cleaning up older development or superseded tags:
```bash
# Delete GitHub release
gh release delete <tag> -y

# Delete remote git tag
git push origin --delete <tag>

# Delete local tag
git tag -d <tag>
```

---

## 8. Report Format to User

Always provide a verified release report:
1. Release tag & title link.
2. Summary of published highlights.
3. CI/CD workflow status (Jobs passed, run time).
4. List of uploaded artifacts with file sizes.
