---
name: rust-clean-code
description: >-
  Enforces strict architecture, readability, and scalability standards for Rust codebases.
  Mandates explicit approval before editing code, zero warning suppressions (no #[allow(...)]),
  no old mod.rs (use foldername.rs), <400 lines/file, <60 lines/fn, max 3 nesting depth,
  role-based architecture (domain/, infra/, api/), DRY, and zero production unwrap()/expect().
---

# `rust-clean-code` Skill: Architecture, Scalability & Quality Rules

Enforces strict production standards for Rust projects. Code must be idiomatic, maintainable, modular, and free of compiler or linter warnings.

---

## 1. Explicit Approval Protocol (Before Editing Code)

**Never** edit any code files or apply modifications without first explaining:
1. **WHY** you are making the proposed modification.
2. **WHAT EFFECT or FIX** it will produce.
3. Receiving **explicit user approval** before modifying any code.

### Operational Boundaries:
- **Initial Proposals & Feature Requests (Approval Mandatory)**: Never silently start mutating or building code without explaining the rationale, impact, and getting confirmation.
- **Autonomous Task Execution & Error Self-Healing (Zero Permission-Seeking)**: Once approved, you own the implementation end-to-end. If an edit triggers a compiler error (`cargo check`), clippy warning (`cargo clippy`), or broken test, fix it autonomously in a closed loop. NEVER ask the user "I made an error / the test failed, may I fix it?"—resolve it autonomously until green.
- **Autonomous Release & CI/CD Self-Healing (Zero Permission-Seeking)**: Once a release is initiated, if GitHub Actions, compilation, or packaging workflows fail due to an error introduced during release/build, fix the issue autonomously without asking permission. Deliver a verified green pipeline.

---

## 2. Zero Tolerance Rules (Non-Negotiable)

1. **No Warning Suppressions**:
   - **Never** add `#[allow(dead_code)]`, `#[allow(unused)]`, or `#[allow(clippy::...)]`.
   - If code triggers a warning, fix the underlying cause immediately.
2. **No Old `mod.rs` (Modern Module Style)**:
   - **Never use `mod.rs`**. Use Rust 2018+ edition file naming: `foldername.rs` placed alongside the `foldername/` directory.
   - Example: `src/domain.rs` alongside `src/domain/user.rs`, NOT `src/domain/mod.rs`.
3. **No Production Panics**:
   - **Never** use `.unwrap()` or `.expect()` in production or library code.
   - Propagate typed errors using `Result<T, E>` and the `?` operator.
   - Reserve `.expect()` strictly for unit test assertions with clear context messages.
4. **Zero Compiler Warnings**:
   - All code must compile cleanly:
     ```bash
     cargo clippy --all-targets -- -D warnings
     cargo fmt --check
     ```
5. **Zero Dead Code**:
   - Unused imports, unused variables, dead functions, and abandoned struct fields must be deleted immediately.

---

## 3. Quantitative Size Limits & Complexity Thresholds

Keep files, functions, and control flow strictly constrained:

| Metric | Soft Limit | Hard Limit | Action if Exceeded |
| :--- | :--- | :--- | :--- |
| **File Length** | 300 lines | **400 lines** | Split into submodules by domain/role |
| **Function Length** | 40 lines | **60 lines** | Extract helper functions or sub-routines |
| **Function Parameters** | 1–3 parameters | **4 parameters** | Group inputs into a dedicated struct / options object |
| **Nesting Depth** | 2 levels | **3 levels** | Guard clauses, early returns, helper decomposition |
| **Files per Folder** | 3–5 files | **7 files** | Subdivide into cohesive domain subfolders |

---

## 4. Role-Based Folder Architecture

Organize `src/` by architectural role rather than flat file dumps:

```
src/
├── main.rs / lib.rs         # Root declarations & entrypoint only
├── error.rs                 # Centralized error enum & conversions
├── domain.rs                # Declares domain submodule
├── domain/                  # Core business entities, models, traits, pure types
│   ├── models.rs
│   └── types.rs
├── infra.rs                 # Declares infra submodule
├── infra/                   # Storage, disk persistence, caching, OS interaction
│   ├── storage.rs
│   └── cache.rs
├── api.rs                   # Declares api submodule
├── api/                     # External HTTP/network clients, REST endpoints
│   ├── client.rs
│   └── payload.rs
├── cli.rs                   # Declares cli submodule
└── cli/                     # CLI arguments, Clap definitions & input validation
    └── args.rs
```

### Module Boundary Invariants:
- **`domain/`**: Must remain pure; zero network or filesystem dependencies.
- **`infra/`**: Encapsulates disk I/O, cache persistence, and hardware interaction.
- **`api/`**: Handles remote communication, returning domain models or typed errors.
- **`cli/`**: Parses input flags and delegates immediately to domain/services.

---

## 5. Readability & DRY (Don't Repeat Yourself)

1. **Centralize Shared Logic**:
   - Reusable string formatting, time parsing, and byte conversions must reside in a dedicated helper module.
   - Constants (URLs, timeouts, default limits) must be declared as module-level `const`s, never hardcoded inline.
2. **Trait-Driven Abstractions**:
   - Abstract I/O and external services behind traits for testability and flexibility.
   - Avoid duplicate `match` arms across multiple files; encapsulate dispatch logic on the enum itself.
3. **Guard Clauses & Early Returns**:
   - Flatten nested `if / else` blocks using early `return Err(...)` or `continue` to stay within the max 3 nesting depth limit.

---

## 6. Pre-Commit Verification Checklist

```bash
# 1. Format code to standard style
cargo fmt

# 2. Verify zero compiler or Clippy warnings
cargo clippy --all-targets -- -D warnings

# 3. Ensure all unit and integration tests pass
cargo test --all-targets
```
