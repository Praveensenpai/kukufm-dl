---
name: karakuri
description: >-
  Audits repositories against clean-code limits (<400 LOC/file, <60 LOC/fn, nesting <=3, 0 clippy warnings),
  auto-generates/synchronizes schema-compliant CODEBASE.md living indexes, detects the project stack to install
  only matching language skills, and syncs custom skills across AI agent environments (~/.gemini, ~/.agents,
  ~/.claude, ~/.codex) using the native karakuri CLI.
---

# `karakuri` Skill: AI Codebase & Skill Repository Orchestrator

The `karakuri` skill instructs AI coding agents on how to leverage the standalone native [`karakuri`](https://github.com/Praveensenpai/karakuri) CLI to enforce architectural guardrails, maintain AI-first living indexes, and ensure cross-agent skill parity.

---

## 1. When to Activate This Skill

Agents MUST invoke `karakuri` commands during the following development lifecycle stages:

1. **Phase 1 Verification (Mandatory Audit)**:
   - Before declaring any coding task, refactor, or bug fix complete, run `karakuri audit .` to guarantee zero line bloat, nesting violations, or clippy warnings.
2. **End-of-Iteration Indexing (Mandatory Digest)**:
   - Whenever source files, structs, enums, or functions are created, renamed, or modified, run `karakuri digest .` to automatically synchronize `CODEBASE.md`.
3. **Multi-Agent Setup & Sync**:
   - When setting up a new machine or updating custom skills, run `karakuri sync` to propagate skills across Antigravity, OpenCode, Claude, and Codex environments.

---

## 2. CLI Command Specifications

### 2.1 Codebase Auditing (`karakuri audit`)
Recursively validates the target directory against production clean-code standards:

```bash
# Audit the current repository against standard limits
karakuri audit .

# Audit an external project
karakuri audit /path/to/project

# Audit with stricter file & function limits
karakuri audit . --max-file-lines 300 --max-fn-lines 40 --max-depth 2

# Audit non-Rust projects without running cargo clippy
karakuri audit . --no-clippy
```

#### What It Enforces:
- **File Length**: Maximum 400 lines per file (flags `FileTooLong`).
- **Function Length**: Maximum 60 lines per function or method (flags `FunctionTooLong`).
- **Nesting Depth**: Maximum 3 nested blocks (flags `NestingTooDeep`).
- **Forbidden Patterns**: Flags `#[allow(dead_code)]` and `#[allow(unused)]`.
- **Documentation**: Flags missing `CODEBASE.md` and missing `README.md`.
- **Compiler / Linter**: Runs `cargo clippy --all-targets -- -D warnings` and captures errors.

### 2.2 Living Semantic Digest (`karakuri digest`)
AST-parses the codebase (using Rust `syn` parser) to extract all public types, structs, enums, signatures, module roles, and line counts:

```bash
# Parse AST and overwrite/update ./CODEBASE.md directly
karakuri digest .

# Output formatted markdown to stdout without writing
karakuri digest . --stdout
```

### 2.3 Cross-Agent Skill Synchronization (`karakuri sync`)
Scans `~/.gemini/config/skills/` and ensures parity across all active agent directories:

```bash
karakuri sync
```

Detected runtimes:
- Antigravity / Gemini: `~/.gemini/config/skills/`
- OpenCode / Standard Agents: `~/.agents/skills/`
- Claude CLI: `~/.claude/skills/`
- Codex CLI: `~/.codex/skills/`

### 2.4 Guardrail Installation (`karakuri install`)
Installs rules and skills headlessly:

```bash
# Install rules into current project (AGENTS.md, GEMINI.md, .agents/rules/)
karakuri install --project --rules -y

# Install rules and all modular skills into current project
karakuri install --project --all -y

# Stack-aware: install generic + only matching language skills (rust, python, mobile, shell)
karakuri install --project --auto -y

# Stack-aware with an explicit override
karakuri install --project --skills --stack rust,python -y

# Install globally across all agent home directories
karakuri install --global --rules -y
```

### 2.5 Stack-Aware Skill Ensure (`karakuri ensure`)

This section is the **authoritative mapping** agents use to decide which skills a project needs. `karakuri ensure` applies it automatically; when the automatic result is wrong, the agent reasons from the table below and overrides it.

`karakuri ensure` reconciles `<project>/.agents/skills` so only the matching skills are present. Idempotent: a second run writes nothing:

```bash
# Ensure the current project has exactly the skills its stack needs
karakuri ensure .

# Ensure an external project
karakuri ensure /path/to/project
```

#### Detection Markers (shallow, root-level)

| Stack | Triggered by |
| :--- | :--- |
| **Rust** | `Cargo.toml` or a top-level `*.rs` file |
| **Python** | `pyproject.toml`, `requirements.txt`, `uv.lock`, `setup.py`, `Pipfile`, or a top-level `*.py` file |
| **Mobile** | `build.gradle(.kts)`, `settings.gradle.kts`, `AndroidManifest.xml`, or a top-level `*.kt` file |
| **Shell** | `Makefile`, `makefile`, or a top-level `*.sh` file |

#### Stack → Skill Mapping

| Stack | Skills installed |
| :--- | :--- |
| **Generic** (always) | `aesthetic-readme-craft`, `git-release-craft`, `git-repo-craft`, `codebase-digest`, `systematic-code-verification`, `karakuri`, `unslop` |
| **Rust** | `rust-clean-code` |
| **Python** | `python-clean-code`, `rust-style-python` |
| **Mobile** | `compose-clean-code` |
| **Shell** | `bash-clean-code` |
| **Optional** (never auto-selected) | `tayori`, `kuroko`, `cli-builder`, `frontend-design`, `web-design-guidelines`, `tui-design` |

Generic guardrails are always included. Python projects receive both `python-clean-code` and `rust-style-python`.

#### Agent Validation Loop

Do not install blindly. On project entry:

1. Scan the root for the markers above and predict the expected skill set from the mapping table.
2. Run `karakuri ensure .` and read the detected stacks and matched skills it prints.
3. Compare the two. If the automatic detection is wrong (a marker is missing, stale, or the repo spans stacks the shallow scan missed), override it:

   ```bash
   karakuri install --project --skills --stack <list> -y
   ```

   where `<list>` is the corrected, comma-separated stacks (for example `--stack rust,python`). `--all` installs every skill; `--auto` is the explicit form of the default.
4. Re-run `karakuri ensure .` after switching branches that change the stack.

---

## 3. Autonomous Closed-Loop Workflows for Agents

### Workflow A: Pre-Completion Verification Loop
Before ending a coding iteration:
1. Run `karakuri audit .`.
2. If violations are reported (exit code 1):
   - Refactor functions exceeding 60 lines by extracting helpers.
   - Refactor nested blocks exceeding depth 3 using early return guard clauses.
   - Fix all clippy warnings.
3. Re-run `karakuri audit .` until terminal output proves:
   `✔ All clean-code rules and documentation standards satisfied!`.

### Workflow B: Mandatory Semantic Index Update Loop
At the end of any iteration touching source code:
1. Run `karakuri digest .`.
2. Verify that `CODEBASE.md` is updated with new modules and signatures.
3. Commit `CODEBASE.md` alongside code changes in the same atomic commit.

### Workflow C: Project Entry Bootstrap
On first entering a repository, follow the **Agent Validation Loop** in §2.5:
1. Scan root markers and predict the expected skill set from the Stack → Skill Mapping.
2. Run `karakuri ensure .` and read the detected stacks and matched skills it prints.
3. If the automatic detection is wrong, override it with `karakuri install --project --skills --stack <list> -y`.
4. Re-run `karakuri ensure .` after switching branches that change the stack.
