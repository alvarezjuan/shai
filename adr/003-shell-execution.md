# ADR-003: Shell Execution via `sh -c`

**Date:** 2026-11-08  
**Status:** Accepted

## Context

Commands may include pipes, redirects, shell builtins, and variable expansion.

## Decision

Execute commands via `tokio::process::Command::new("sh").arg("-c").arg(command)`.

## Rationale

- Supports pipes (`ls | grep foo`), redirects (`> file`), builtins (`cd`, `export`)
- Matches user mental model of "running a shell command"
- Simpler than parsing and executing pipeline manually

## Consequences

- Security: shell injection risk if command not validated (mitigated by risk classifier)
- Slightly heavier than direct `Command::new(cmd).args(args)`
- Windows compatibility requires separate handling (future work)

## Related

- [ADR-001: LLM-Only Risk](001-llm-only-risk.md)
- [ADR-006: Dual LLM for Risk](006-dual-llm-risk.md)