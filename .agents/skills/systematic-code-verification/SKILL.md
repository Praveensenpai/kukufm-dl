---
name: systematic-code-verification
description: >-
  Enforces rigorous, evidence-based code verification before declaring any task complete.
  Mandates a strict zero-assumption policy (code is presumed broken until proven working),
  a 3-phase verification cycle (static analysis, automated test suites, and live runtime smoke tests),
  explicit proof presentation with real terminal outputs, and autonomous closed-loop self-healing.
---

# `systematic-code-verification` Skill: Active Proof & Verification Protocol

Eliminates the "fire-and-forget" assumption antipattern. Enforces rigorous, evidence-based code verification before any task or bugfix is declared complete.

---

## 1. Core Philosophy: Presume Broken Until Proven

> [!IMPORTANT]
> **Zero Assumptions Rule**: An unexecuted line of code is an unverified line of code. Code is presumed broken until proven working by active execution and real terminal output.

### The Antipattern We Prohibit:
- **Fire-and-Forget**: Editing code files, declaring "I fixed the issue", and stopping without running the code, tests, or linters.
- **Theoretical Correctness**: Assuming code works because "the syntax looks fine" or "the logic seems sound".
- **Passing the Burden to the User**: Telling the user "You can test it by running X" when the agent possesses terminal execution tools to test it immediately.

---

## 2. The 3-Phase Verification Cycle (Mandatory Before Completion)

Every code change, bugfix, or feature implementation must pass through all applicable phases of the verification cycle before completion:

```text
┌───────────────────────┐     ┌───────────────────────┐     ┌───────────────────────┐
│        Phase 1        │     │        Phase 2        │     │        Phase 3        │
│   Static Validation   │ ──> │    Test Execution     │ ──> │  Live Runtime Smoke   │
│  (Compile, Lint, Typ) │     │ (Unit & Integrations) │     │  (CLI, Exec, Output)  │
└───────────────────────┘     └───────────────────────┘     └───────────────────────┘
```

### Phase 1: Static Validation & Compilation
Code must build cleanly with **zero warnings and zero errors**:

| Language / Stack | Mandatory Static Check Commands |
| :--- | :--- |
| **Rust** | `cargo check --all-targets`<br>`cargo clippy --all-targets -- -D warnings`<br>`cargo fmt --check` |
| **Python** | `uv run ruff check`<br>`uv run mypy .`<br>`uv run ruff format --check` |
| **TypeScript / JS** | `npx tsc --noEmit`<br>`npm run lint` or `npx eslint .` |
| **Go** | `go vet ./...`<br>`golangci-lint run` |
| **Bash / Shell** | `shellcheck -x <script.sh>` |
| **C / C++** | `cmake --build build -- -Wall -Wextra -Werror` |

### Phase 2: Automated Test Execution
Run automated test suites to ensure zero regressions and positive validation:
- **Rust**: `cargo test --all-targets`
- **Python**: `uv run pytest -v`
- **TypeScript / JS**: `npm test` or `pnpm test` or `bun test`
- **Go**: `go test -v ./...`
- **Coverage Requirement**: If modifying existing logic or fixing a bug, verify that existing tests pass. If no test covers the modified path, write a targeted unit or integration test before finishing.

### Phase 3: Live Runtime & Smoke Verification
Passing tests alone is not sufficient if the application fails at runtime:
1. **Direct Execution**: Actively invoke the executable binary, script, or CLI with representative real-world flags and inputs.
2. **Exit Code Inspection**: Confirm that the process exits with `code 0` (or the expected non-zero code for negative test cases).
3. **Output & Side-Effect Validation**:
   - Inspect stdout/stderr for expected output shape, clean formatting, or JSON validity.
   - If the program writes to disk or modifies state, inspect the created files (`cat`, `ls -l`, `view_file`) to ensure contents match specifications.
4. **Boundary & Negative Testing**: Execute at least one edge case (e.g. invalid arguments, missing files, empty inputs) to verify graceful error handling.

---

## 3. Evidence-Based Completion Standard

An agent must **never** claim success with bare assertions like *"I have updated the code and fixed the issue."*

### Mandatory Proof Output:
Every completion message must explicitly show the evidence of verification:
1. **The Exact Commands Run**: Display the verification commands executed.
2. **Terminal Output Summary**: Quote or summarize the actual test counts, compiler exit status, or smoke test output.
3. **Artifact / Side-Effect Confirmation**: Point to verified file modifications or generated outputs.

#### Example Proof Block:
```text
✅ Phase 1 (Static): cargo clippy --all-targets -- -D warnings -> 0 warnings, clean.
✅ Phase 2 (Tests):  cargo test -> 14 passed; 0 failed; 0 ignored.
✅ Phase 3 (Smoke):  ./target/debug/app --input sample.json -> Process exited 0, generated output verified.
```

---

## 4. Autonomous Closed-Loop Self-Healing

> [!NOTE]
> **Zero Permission-Seeking on Failures**: Once a task or fix has been approved by the user, the agent owns the outcome end-to-end.

If Phase 1, Phase 2, or Phase 3 fails during verification:
1. **Do NOT stop and ask the user**: Never ask *"I encountered an error / the test failed, may I fix it?"*.
2. **Diagnose Autonomously**: Read the compiler error, test failure traceback, or runtime panic log.
3. **Remediate in Place**: Fix the root cause in the source code.
4. **Re-Verify the Cycle**: Re-run Phase 1, 2, and 3 until the entire verification pipeline is clean green.

---

## 5. Prohibited Excuses & Antipatterns

| Prohibited Phrase / Excuse | Why It Is Rejected | Required Action |
| :--- | :--- | :--- |
| *"This should work now."* | "Should" expresses unverified hope. | Run the code and prove it works. |
| *"The code looks correct theoretically."* | Syntax visual checks do not catch runtime bugs. | Execute the test suite and verify behavior. |
| *"You can test it by running `command`."* | Passes the testing burden to the user. | Run the command yourself using available execution tools. |
| *Modifying tests to make them pass trivially.* | Masking failures by weakening test assertions. | Fix the implementation to satisfy the original contract. |
