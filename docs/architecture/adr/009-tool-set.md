# ADR-009: Tool Set v1 — Read/Write/Inspect/Web/Git

**Date:** 2026-11-08  
**Status:** Accepted

## Context

The agent needs tools beyond `shell` to be useful: file inspection, web search, git operations, file editing, task tracking.

## Decision

**v1 Tools** (all implemented with `#[rig_tool]`):
- **Read-only (Level 1):** `read`, `glob`, `grep`, `list_dir`, `web_search`, `web_fetch`, `git_status`, `git_diff`, `git_log`, `todo_read`, `todo_write`
- **Write (Level 3):** `git_commit`, `git_push`, `edit`, `patch`
- **Shell (Level 2–4):** `shell` (LLM-classified)

## Rationale

- Read-only tools need no confirmation — safe to auto-execute
- Write tools (git commit/push, edit) require confirmation (Level 3)
- Web search/fetch are read-only — Level 1
- `todo_read`/`todo_write` for agent task tracking — Level 1
- All tools use same dispatch pattern in manual loop (Option B)

## Consequences

- 15 tools in v1 — significant but each is small
- Risk classification only for `shell` (others have fixed levels)
- Git write tools need user confirmation
- File edit tools need careful diff preview before confirm

## Related

- [ADR-002: Manual Agent Loop](002-manual-agent-loop.md)
- [ADR-003: Shell Execution](003-shell-execution.md)
- [ADR-006: Dual LLM](006-dual-llm-risk.md)