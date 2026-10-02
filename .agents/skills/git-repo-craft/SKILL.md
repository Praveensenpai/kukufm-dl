---
name: git-repo-craft
description: >-
  Automates GitHub repository creation, configuration, and metadata auditing. Mandates minimal
  yet meaningful repository descriptions, curated and standardized GitHub topics, clean README and
  license initialization, and active verification via the GitHub CLI (gh).
---

# `git-repo-craft` Skill: Repository Creation, Editing & Metadata Crafting

Standardizes GitHub repository creation, editing, and metadata auditing. Guarantees that every repository has a minimal yet meaningful description, curated discoverable topics, clean initialization, and verified configuration.

---

## 1. The Core Philosophy: Minimal Yet Meaningful

A repository's GitHub metadata is its front door. Poor metadata kills discoverability and confuses visitors.

1. **Minimal**: No buzzword soup, no filler ("This is a repo that contains..."), no redundant repository name repetition. Keep it under 80 characters or one crisp, punchy sentence.
2. **Meaningful**: Immediately conveys **WHAT** the project is and **WHAT PROBLEM** it solves.
3. **Discoverable**: 4–8 curated, kebab-case topics covering domain, language, purpose, and ecosystem.

---

## 2. Description Crafting Standards

### Rules for Repository Descriptions:
- **Maximum Length**: Aim for 50–90 characters (hard limit: 120 characters).
- **No Redundancy**: Never start with the repo name (e.g., `karakuri — A tool for...`). The repo name is already visible above the description.
- **No Filler**: Never use phrases like:
  - ❌ "A repository for..."
  - ❌ "This project is intended to..."
  - ❌ "Collection of useful scripts and tools..."
- **Action & Value Pattern**:
  - `[Adjective] [Core Entity] for [Primary Target / Use Case]`
  - `[Primary Action Verb] [Object] with [Key Differentiator]`

### Good vs. Bad Examples:
| Status | Description | Rationale |
| :--- | :--- | :--- |
| ❌ Bad | `my-repo - A repository that contains various miscellaneous scripts and tools` | Redundant name, filler phrasing, vague |
| ❌ Bad | `A collection of python scripts for my daily tasks and work` | Generic, zero meaning, filler phrasing |
| ✅ Good | `Modular skills and automated workflows for AI coding agents` | Minimal, precise domain, immediate value |
| ✅ Good | `Fast Rust compiler offloader to Kaggle Cloud with zero local CPU load` | Clear value proposition, concise |
| ✅ Good | `Production standards, linting, and quality guardrails for Python` | Punchy, informative, clean |

---

## 3. Curated Topics Matrix

Every repository must have **4 to 8** topics categorized across four dimensions:

| Dimension | Purpose | Examples |
| :--- | :--- | :--- |
| **Domain** | Primary field or area | `ai-agents`, `cloud-computing`, `systems-programming`, `web-development` |
| **Language / Stack** | Primary programming language | `rust`, `python`, `bash`, `typescript`, `flutter` |
| **Purpose / Function** | What the tool actually does | `agent-skills`, `workflow-automation`, `developer-tools`, `cli`, `linter` |
| **Ecosystem** | Surrounding framework / tool | `antigravity`, `github-actions`, `docker`, `kaggle` |

### Topic Formatting Rules:
- **Strictly Kebab-Case**: `developer-tools`, never `developer_tools` or `DeveloperTools`.
- **All Lowercase**: `ai-agents`, never `AI-Agents`.
- **No Punctuation or Spaces**: Letters, numbers, and hyphens only.
- **Standard Abbreviations**: Use recognized industry tags (`cli`, `api`, `sdk`, `tui`).

---

## 4. Repository Audit & Editing Workflow

When tasked with inspecting, cleaning, or updating an existing repository:

### Step 1: Inspect Current Metadata
```bash
gh repo view <owner>/<repo> --json description,repositoryTopics,homepage,visibility,defaultBranchRef
```

### Step 2: Audit Description
- If **missing or empty**: Analyze repository files (README, package manifests) to craft a minimal, punchy one-liner.
- If **bloated or redundant**: Strip filler words and repository name prefix.
- Update description:
  ```bash
  gh repo edit <owner>/<repo> -d "<Minimal yet meaningful description>"
  ```

### Step 3: Audit & Apply Topics
- Check if topics are missing or suboptimal.
- Select 4–8 relevant topics from the codebase inspection.
- Apply topics:
  ```bash
  gh repo edit <owner>/<repo> \
    --add-topic topic-one \
    --add-topic topic-two \
    --add-topic topic-three \
    --add-topic topic-four
  ```
  *(To remove obsolete topics: use `--remove-topic <old-topic>`)*

### Step 4: Verify Updates
```bash
gh repo view <owner>/<repo> --json description,repositoryTopics
```

---

## 5. Repository Creation Workflow (`gh repo create`)

When creating a new repository from scratch or an existing local directory:

### Step 1: Pre-Flight Checklist
1. Ensure `.gitignore` exists and matches the project language stack.
2. Ensure an open-source `LICENSE` (e.g., MIT, Apache-2.0) is present.
3. Ensure a clean `README.md` with project title and brief overview exists.
4. Verify initial commit is cleanly staged and committed.

### Step 2: Initialize & Push to GitHub
```bash
# In the project directory:
gh repo create <repo-name> \
  --public \
  --description "<Minimal yet meaningful description>" \
  --source=. \
  --remote=origin \
  --push
```
*(Use `--private` if internal or confidential).*

### Step 3: Immediately Set Curated Topics
```bash
gh repo edit \
  --add-topic <domain-topic> \
  --add-topic <language-topic> \
  --add-topic <function-topic> \
  --add-topic <ecosystem-topic>
```

### Step 4: Verify Final State
```bash
gh repo view --json description,repositoryTopics,url
```
Confirm the output displays the clean description and all topics without error.
