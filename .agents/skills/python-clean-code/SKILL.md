---
name: python-clean-code
description: >-
  Enforces strict architecture, modern type hints, clean code standards, and quality tooling
  for Python codebases. Mandates exclusive use of the uv package manager, 100% type annotations,
  automated ruff linting, isort import sorting, formatting on every change, zero dead code,
  strict file/function size limits, and role-based folder hierarchy.
---

# `python-clean-code` Skill: Quality, Typing & Architecture Standards

Enforces high-rigor production standards for Python projects. Code must be strictly typed, formatted, organized into role-based submodules, and managed exclusively via `uv`.

---

## 1. Package Management via `uv` (Mandatory)

**Never** use raw `pip install`, `poetry`, or `pipenv`. Always manage dependencies and environments through `uv`:

```bash
# Initialize a project or add dependencies
uv init
uv add <package>
uv add --dev ruff mypy pytest

# Execute commands within virtualenv
uv run <command>
uv run python -m <module>

# Sync dependencies
uv sync
```

---

## 2. On Every Change: Lint, Sort Imports & Format

Always execute automated quality checks on any file edit:

```bash
# 1. Lint and auto-fix violations (including unused imports, code smells)
uv run ruff check --fix

# 2. Sort imports according to isort standard
uv run ruff check --select I --fix

# 3. Format code style (black-compatible, 88 char line length)
uv run ruff format

# 4. Strict static type check
uv run mypy .
```

### Zero-Tolerance Rules:
- **No `# noqa`** or **`# type: ignore`** suppressions without an explicit comment explaining the unfixable upstream reason.
- **Zero dead code**: Unused imports, unreachable blocks, and abandoned helper functions must be deleted immediately.

---

## 3. 100% Modern Type Hinting

All code must include explicit type annotations on variables, function signatures, and return values:

1. **Modern Union & Collection Syntax (Python 3.10+)**:
   - Use `int | str | None` instead of `Optional[Union[int, str]]`.
   - Use built-in generics: `list[str]`, `dict[str, Any]`, `tuple[int, ...]`, `set[Path]`.
2. **Explicit Return Types**:
   - Every function and method must declare its return type: `def process(...) -> None:`.
   - Generator functions must declare `Iterator[T]` or `Generator[Y, S, R]`.
3. **Structured Data**:
   - Avoid untyped nested dictionaries.
   - Prefer `@dataclass(frozen=True, slots=True)` or `pydantic.BaseModel` for data transfer objects.

---

## 4. Quantitative Size Limits & Modularity

Keep modules and functions focused on a single responsibility:

| Metric | Target | Hard Limit | Action if Exceeded |
| :--- | :--- | :--- | :--- |
| **File Length** | 100–200 lines | **300 lines** | Decompose into submodules by domain/role |
| **Function Length** | 15–30 lines | **45 lines** | Extract sub-routines or pipeline steps |
| **Functions per File** | 4–6 functions | **8 functions** | Separate responsibilities into dedicated files |
| **Files per Subpackage**| 3–5 files | **7 files** | Group related logic into a subfolder with `__init__.py` |
| **Function Parameters** | 1–3 parameters | **4 parameters** | Encapsulate parameters in a `@dataclass` or config model |

---

## 5. Role-Based Directory Architecture

Structure packages by responsibility instead of a flat dump of scripts:

```
src/<package_name>/
├── __init__.py
├── __main__.py              # Entrypoint invocation (python -m <pkg>)
├── exceptions.py            # Domain-specific typed exception classes
├── cli/                     # CLI parsing (Click, Typer, or Argparse)
│   ├── __init__.py
│   └── main.py
├── models/                  # Pure data structures (Dataclasses / Pydantic)
│   ├── __init__.py
│   └── entity.py
├── services/                # Core business logic and orchestration
│   ├── __init__.py
│   └── processor.py
├── client/ (or api/)        # External HTTP/network clients (httpx, requests)
│   ├── __init__.py
│   └── remote.py
├── storage/ (or db/)        # File persistence, SQLite, caching
│   ├── __init__.py
│   └── repository.py
└── utils/                   # Reusable formatters, time helpers, validators
    ├── __init__.py
    └── format.py
```

### Module Boundaries:
- **`models`** must never perform network or disk I/O.
- **`storage`** encapsulates file read/write operations and schema migrations.
- **`services`** coordinates business workflows by calling clients and storage.
- **`cli`** only parses input and presents output; no domain logic inside CLI handlers.

---

## 6. DRY & Modern Idioms

1. **Pathlib over `os.path`**: Always use `pathlib.Path` for file manipulation, path joining, and existence checks.
2. **Context Managers**: Always use `with` statements for files, sockets, locks, and network sessions to guarantee resource release.
3. **Structured Exceptions**: Define clear, custom exceptions in `exceptions.py` inheriting from `Exception` rather than raising generic `RuntimeError` or `ValueError`.
4. **Pattern Matching**: Use `match / case` statements where appropriate instead of long chains of `if/elif isinstance(...)`.

---

## 7. Pre-Commit Verification Checklist

```bash
# Auto-fix linting & import sorting
uv run ruff check --fix
uv run ruff check --select I --fix

# Format code
uv run ruff format

# Static type verification
uv run mypy .

# Run test suite
uv run pytest
```
