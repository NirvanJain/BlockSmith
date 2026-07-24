---
name: codebase-review
description: Systematically explore an unfamiliar or partially-known codebase and produce a structured architecture summary covering tech stack, directory layout, key modules, data flow, and notable patterns.
---

# Codebase Review

Systematically explore a codebase the user hasn't seen before (or hasn't fully mapped) and produce a clear, structured summary. The goal is to give the user — and future agents — a reliable mental model of the project without re-reading everything from scratch.

## When to use

- User says "go through the codebase", "explore the project", "what's the architecture", or similar.
- Starting work on a project you don't have full context on.
- Re-orienting after a long gap between sessions.

## Procedure

### Phase 1 — Top-level survey

1. Read the project root directory. Identify the language(s), framework(s), and build system from config files (Cargo.toml, package.json, pyproject.toml, go.mod, Makefile, etc.).
2. Read README, CONTRIBUTING, or similar docs if present — extract project purpose and setup instructions.
3. Map the directory tree (one level deep per directory) to understand the layout.

### Phase 2 — Backend / core logic

1. Find the entry point (main, app, index, etc.) and read it.
2. Read the module/index file that declares internal modules (lib.rs, __init__.py, etc.).
3. Identify the routing/API layer, data models, database access, auth, and any domain-specific modules (e.g., blockchain, ML, payments).
4. Read one representative file from each major module to understand the code style and patterns.

### Phase 3 — Frontend (if present)

1. Read the framework entry point (App.tsx, App.vue, etc.) and layout/routing structure.
2. Identify the component tree and shared state management.
3. Check the API client layer to understand how frontend talks to backend.

### Phase 4 — Infrastructure & config

1. Read migration files, Dockerfiles, CI configs, or deployment scripts.
2. Check .env.example for required environment variables.
3. Note any notable patterns: mock data, feature flags, A/B testing, etc.

### Phase 5 — Synthesize

Produce a structured summary with these sections:

```
## Tech stack
- Language, framework, database, auth, key libraries

## Directory layout
- Brief tree with one-line purpose per directory

## Architecture
- How the app starts, request flow, data flow
- Key abstractions and patterns

## Notable decisions
- Anything surprising, unconventional, or worth flagging

## Open questions
- Things that are unclear or worth investigating further
```

## Tips

- Don't read every file. Read enough to map the shape, then dive into specifics only when asked.
- Use `glob` to find files by pattern rather than reading entire directories.
- If the project has tests, glance at one to understand testing patterns.
- If there's a database schema (migrations, SQL files), read it — it's the most reliable source of truth for data model.
- Note branding inconsistencies, stale references, or placeholder code — these are common sources of bugs.
