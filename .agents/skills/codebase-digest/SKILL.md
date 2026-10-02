---
name: codebase-digest
description: >-
  Generates and maintains a high-density, AI-first CODEBASE.md index of the project.
  Optimized specifically for LLM context injection to understand architecture, public symbols,
  signatures, and module dependencies in a single file without reading individual source files.
  Mandates automated synchronization at the end of every code modification iteration.
---

# `codebase-digest` Skill: AI-First Semantic Codebase Index

Maintains a single-file, token-dense architectural and semantic index of the project called `CODEBASE.md`. Designed specifically for AI and LLM agents to achieve 100% codebase comprehension in a single read without burning tool calls or reading raw source files.

---

## 1. Core Philosophy: AI-First, Zero Fluff

`CODEBASE.md` is **not** documentation for end users or a marketing README. It is an **executable mental model** for AI agents:
1. **Maximum Information Density**: No conversational prose, marketing adjectives, or narrative filler.
2. **Type & Signature Accuracy**: Exact struct fields, enum variants, and function signatures.
3. **Dependency & Call Graph**: Clear mapping of what imports what and who calls whom.
4. **Living Synchronization**: Must be updated at the end of every iteration where codebase files change.

---

## 2. Mandatory End-of-Iteration Update Protocol

Whenever an agent adds, edits, refactors, renames, or deletes any code file:
- **Never end the turn without updating `CODEBASE.md`**.
- The agent must update the corresponding file entries, symbol signatures, and line counts before declaring the task complete.
- When an agent first touches a repository that lacks `CODEBASE.md`, it must generate it immediately as part of the initial task.

---

## 3. Standard `CODEBASE.md` Schema

Every `CODEBASE.md` file must strictly follow this dense structure:

```markdown
# CODEBASE.md: <Project Name> Semantic Digest

> **Notice**: This file is an AI-optimized semantic index. Do not write narrative prose. Keep token density high.

## 1. System Topology & Data Flow
\`\`\`text
<Entrypoint> ──> <Parser/CLI> ──> <Core/Dispatcher> ──> <Domain Logic> ──> <Infra/IO>
\`\`\`

## 2. Global Constraints & Architecture Patterns
- **Primary Language & Edition**: (e.g. Rust 2021 edition / Python 3.12 uv)
- **Architectural Paradigm**: (e.g. Role-based: domain/, infra/, api/)
- **Hard Constraints**: <400 lines/file, <60 lines/fn, zero production unwrap(), 0 warnings.
- **Target Distribution**: (e.g. Linux x86_64 standalone binary via GitHub Releases)

## 3. Module & Interface Skeleton

### `<relative/path/to/file>` (Role: <domain|infra|api|cli|tui>, Lines: <count>)
- **Responsibility**: Single concise sentence.
- **Imports**: `crate::...`, `external_crate::...`
- **Types & Enums**:
  \`\`\`<lang>
  pub struct Example { pub field: Type }
  pub enum Mode { VariantA, VariantB(String) }
  \`\`\`
- **Public Functions & Signatures**:
  \`\`\`<lang>
  pub fn process_input(arg: &Arg) -> Result<Output, Error>
  \`\`\`
- **Consumers**: (modules/functions that invoke this file)
- **Side Effects / I/O**: (filesystem writes, network requests, env vars)

## 4. Execution Lifecycle Trace
1. **Startup**: Entrypoint (`main.rs`) initializes logger and parses CLI arguments.
2. **Dispatch**: CLI arguments route to specific command handlers in `cli/`.
3. **Execution**: Command handlers call domain logic in `domain/`.
4. **I/O & Persistence**: Domain logic delegates to `infra/` for disk/network I/O.
5. **Exit**: Graceful shutdown with typed error codes.

## 5. Verification Commands
\`\`\`bash
# Build
cargo build --release --target x86_64-unknown-linux-gnu

# Test
cargo test --all-targets

# Lint & Format
cargo clippy --all-targets -- -D warnings
cargo fmt --check
\`\`\`

## 6. Recent Iteration Changes
- **YYYY-MM-DD**: Concise delta of what files were added, modified, or removed.
```

---

## 4. Updating Guidelines for Agents

1. **Keep Signatures Exact**:
   When a function signature changes in code, reflect the exact new signature in `CODEBASE.md`.
2. **Update Line Counts**:
   Keep approximate file line counts up to date so the LLM knows file weight and complexity.
3. **Prune Stale Entries**:
   When files are deleted or renamed, immediately prune them from `CODEBASE.md`.
4. **Atomic Commits**:
   Commit `CODEBASE.md` alongside the code changes in the same atomic commit whenever possible.
