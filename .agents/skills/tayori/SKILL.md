---
name: tayori
description: >-
  Sends aesthetic Telegram notifications, interactive approval requests, and completion notices to the user's phone via Tayori (便り).
  Use 'tayori ask -i' (strictly with -i) when awaiting user approval or asking critical questions, 'tayori done' when a task finishes,
  and 'tayori alert' for important warnings or errors.
---

# `tayori` (便り) Notification Skill

Integrates AI coding assistants with Telegram notifications via the `tayori` CLI.

## When to Use
- **Awaiting Approval (Interactive Required)**: When pausing for human confirmation before running destructive actions, major migrations, or architectural decisions. **Mandatory**: Always pass `-i` (`--interactive`) so the CLI blocks until the user taps `[Approve]` or `[Reject]` on Telegram.
- **Task Milestones & Completion**: When finishing a multi-step task, build, or release so the user knows immediately (`tayori done`).
- **Informational Notifications**: One-way status messages or progress updates (`tayori send`). Never use `tayori ask` for one-way messages.
- **Errors & Warnings**: High-priority errors or build failures requiring user attention (`tayori alert`).

## Usage

```bash
# 1. Interactive Mobile Approval (Mandatory: blocks until user taps [Approve] or [Reject])
tayori ask -i "Proposed English and Romaji search support for app titles. Awaiting approval to proceed."

# With custom timeout in seconds (default is 300s)
tayori ask -i --timeout 600 "Run destructive database migration on production?"

# 2. Informational One-Way Notification
tayori send "Starting background compilation for release v1.0.0..."

# 3. Task Completion Alert
tayori done "Release v1.0.0 successfully published and CI is green!"

# 4. Warning or Error Alert
tayori alert "Database connection failed after 3 retries" --level error

# 5. Pipe Command Logs
cargo test 2>&1 | tayori pipe --title "Cargo Test"
```

## Approval Protocol Rules
- **Never Run `tayori ask` Without `-i`**: Fire-and-forget approval questions exit immediately without waiting for human input, defeating the approval protocol.
- **Exit Code Verification**:
  - Exit code `0`: User tapped `[Approve]`. Safe to proceed.
  - Non-zero exit code: User tapped `[Reject]` or the request timed out. Do not execute proposed modifications.

