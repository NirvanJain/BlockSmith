---
name: issue-triage
description: Given a GitHub issue URL, read the issue, explore the relevant codebase, and identify exactly which files need to change and what changes are needed.
---

# Issue Triage

Given a GitHub issue (URL or description), read the issue, explore the codebase, and produce a precise list of files that need to change along with what needs to happen in each.

## When to use

- User pastes a GitHub issue link and says "which files need to change", "go through this", or similar.
- Starting work on an open-source contribution.
- Reviewing an issue before assigning or estimating effort.

## Procedure

### Phase 1 — Understand the issue

1. Fetch the issue using `gh issue view <number> -R <owner>/<repo>` or read the URL directly.
2. Extract: what's broken or requested, expected behavior, any error messages, and which part of the codebase the reporter mentions.
3. If the issue references specific files or functions, note them — but verify independently (reporters are sometimes wrong).

### Phase 2 — Map the codebase

1. Explore the project structure (use the codebase-review skill if you don't already have context).
2. Identify the area of the codebase the issue touches: frontend, backend, tests, config, etc.
3. Find the specific files mentioned in the issue (or their equivalents).

### Phase 3 — Trace the impact

1. Read the affected files. Understand what they do and how they connect to other parts.
2. Search for references to the affected functions, types, or modules using grep/glob.
3. Check if there are tests for the affected code — note whether tests exist and whether they need updating.
4. Look for related patterns elsewhere in the codebase that might need the same fix.

### Phase 4 — Produce the triage report

```
## Issue summary
- One-line description of what the issue is about

## Affected files
- `path/to/file.rs` — what needs to change and why
- `path/to/other.ts` — what needs to change and why

## Impact analysis
- Which other files reference the affected code
- Whether tests exist and need updating
- Whether the change could break other functionality

## Suggested approach
- Step-by-step plan for implementing the fix
- Any decisions that need user input

## Risk level
- Low / Medium / High — based on how many files are touched and how core the code is
```

## Tips

- Don't just trust the issue description — trace the actual code path.
- Check git blame or recent commits on affected files to understand recent changes.
- If the issue is vague, look at the reporter's environment (browser, OS, version) and try to reproduce mentally.
- For cross-project contributions, read the project's CONTRIBUTING.md or style guide first.
- When in doubt, list more files than fewer — it's better to over-triage than to miss something.
