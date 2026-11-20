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

### Antipatterns We Prohibit:
- **The Unit-Test Fallacy (`test.ts/py/rs` Is NOT Enough)**: Assuming that passing test files (`test.ts`, `test.py`, `test.rs`, `pytest`, `cargo test`) proves the feature or bugfix works. Unit tests run in vitro with mocks, stubs, and synthetic inputs. They do NOT verify compiled binary execution, CLI argument parsing, environment variable handling, real disk I/O, network behavior, or end-to-end integration. Passing unit tests is merely a Phase 2 gate, NEVER sufficient proof on its own.
- **Fire-and-Forget**: Editing code files, declaring "I fixed the issue", and stopping without actively running the code, tests, or linters.
- **Theoretical Correctness**: Assuming code works because "the syntax looks fine" or "the logic seems sound".
- **Unverified Assertions**: Claiming "verified" or "works as expected" without displaying the actual terminal commands, stdout, stderr, and exit codes.
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
- **Coverage Requirement**: If modifying existing logic or fixing a bug, verify that existing tests pass. If no test covers the modified path, write a targeted unit or integration test.
- **Limitation Notice**: Automated tests only validate isolated logic in vitro. Passing Phase 2 NEVER excuses or replaces Phase 3.

### Phase 3: Live Runtime Smoke Verification (Mandatory & Non-Negotiable)
Passing test files (`test.ts`, `test.py`, `test.rs`) is never enough if the application fails in real execution. Phase 3 is mandatory for EVERY code change:
1. **Direct Execution**: Actively invoke the compiled binary, CLI executable, or entrypoint script in the terminal with realistic, representative flags and inputs.
2. **Exit Code & Output Inspection**: Confirm that the process exits with the expected code (`code 0` for success or expected non-zero code for negative test cases). Inspect stdout and stderr for correct structure, clean formatting, and expected output.
3. **Inspect Output & Side Effects**:
   - If the program creates or modifies files, actively inspect the results (`cat`, `head`, `ls -la`, `view_file`) and display the proof.
   - For web services or APIs, execute real HTTP requests (`curl`, `urllib`, etc.) and verify status codes and response bodies.
4. **Boundary & Negative Testing**: Execute at least one edge case (e.g. invalid arguments, missing files, malformed inputs) to verify graceful error handling.

---

## 3. Evidence-Based Completion Standard: Show What You Verified

An agent must **never** claim success with bare assertions like *"I have updated the code and fixed the issue"* or vague handwaves like *"Phase 3: verified"*.

### Mandatory Proof Output:
Every completion message must explicitly show the evidence of verification:
1. **The Exact Commands Run**: Display the verification commands executed in the terminal.
2. **Actual Terminal Output**: Paste the verbatim or trimmed terminal output showing exit codes, stdout, stderr, or test summaries. NEVER conceal output behind bare checkmarks.
3. **Artifact / Side-Effect Confirmation**: Display the output of inspecting generated files or side effects to prove the runtime change took effect.

#### Example Proof Block:
```text
✅ Phase 1 (Static Validation):
   Command: cargo clippy --all-targets -- -D warnings
   Output:  Checking karakuri v0.6.5 (/path/to/karakuri)
            Finished `dev` profile [unoptimized + debuginfo] target(s) in 1.42s (0 warnings, clean)

✅ Phase 2 (Test Execution):
   Command: cargo test --all-targets
   Output:  test result: ok. 18 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

✅ Phase 3 (Live Runtime Smoke Test):
   Command: ./target/debug/karakuri audit .
   Output:  Exit code: 0
            ┌──────────────────────────────────────────────┐
            │ Karakuri Architecture & Clean-Code Audit     │
            │ All clean-code rules passed. 0 violations.   │
            └──────────────────────────────────────────────┘
   Side-Effect Inspection: Verified modified files via diff; no stray temp files.
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
| *"All tests passed in test.py/ts/rs so it works."* | Unit tests run in vitro; they do not prove runtime CLI, binary execution, or live behavior. | Execute the live binary/script in Phase 3 with real inputs. |
| *"Phase 3 verified." (without terminal output)* | Stating verification without showing actual terminal output is an unverified assumption. | Display the verbatim command, exit code, and stdout/stderr. |
| *"This should work now."* | "Should" expresses unverified hope. | Run the code live and prove it works with real terminal output. |
| *"The code looks correct theoretically."* | Syntax visual checks do not catch runtime bugs. | Execute the test suite and the live runtime binary. |
| *"You can test it by running `command`."* | Passes the testing burden to the user. | Run the command yourself using available execution tools. |
| *Modifying tests to make them pass trivially.* | Masking failures by weakening test assertions. | Fix the implementation to satisfy the original contract. |
