# ADR-004: Per-Directory History (`.shai/session.jsonl`)

**Date:** 2026-11-08  
**Status:** Accepted

## Context

Conversation history must persist across invocations for both one-shot and REPL modes.

## Decision

Store history as JSONL in `.shai/session.jsonl` relative to current working directory.

## Rationale

- Project-scoped sessions match developer workflow (one session per repo)
- JSONL is append-only, portable, human-readable
- One `rig::completion::Message` per line
- `.shai/` directory also holds future config/cache

## Consequences

- Multiple projects = multiple independent histories
- No global history by default (can add flag later)
- History grows unbounded — `max_history_turns` config limits loaded context

## Related

- [ADR-010: RAG v2](010-rag-v2.md)
- [ADR-002: Manual Agent Loop](002-manual-agent-loop.md)