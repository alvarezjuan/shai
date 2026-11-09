# Architecture Decision Records (ADR)

This folder contains 11 individual ADR files documenting key architectural decisions for **shai**.

## ADR Index

| ID | Title | Status | Date |
|----|-------|--------|------|
| [001](001-llm-only-risk.md) | LLM-Only Risk Classification | Accepted | 2026-11-08 |
| [002](002-manual-agent-loop.md) | Manual Turn-by-Turn Agent Loop | Accepted | 2026-11-08 |
| [003](003-shell-execution.md) | Shell Execution via `sh -c` | Accepted | 2026-11-08 |
| [004](004-per-directory-history.md) | Per-Directory History (`.shai/session.jsonl`) | Accepted | 2026-11-08 |
| [005](005-non-streaming.md) | Non-Streaming for v1 | Accepted | 2026-11-08 |
| [006](006-dual-llm-risk.md) | Dual LLM for Risk Assessment | Accepted | 2026-11-08 |
| [007](007-rig-umbrella.md) | rig 0.46 Umbrella Crate | Accepted | 2026-11-08 |
| [008](008-provider-config.md) | Provider Config via TOML + Env Vars | Accepted | 2026-11-08 |
| [009](009-tool-set.md) | Tool Set v1 — Read/Write/Inspect/Web/Git | Accepted | 2026-11-08 |
| [010](010-rag-v2.md) | RAG Integration v2 — Local Per-Directory Index | Accepted (Deferred) | 2026-11-08 |
| [011](011-ssh-v2.md) | SSH Integration v2 — Key-Based Only | Accepted (Deferred) | 2026-11-08 |

---

## Quick Links by Category

### Core Architecture
- [ADR-001](001-llm-only-risk.md): Risk classification approach
- [ADR-002](002-manual-agent-loop.md): Agent loop design
- [ADR-004](004-per-directory-history.md): History storage

### Configuration & Providers
- [ADR-006](006-dual-llm-risk.md): Dual LLM setup
- [ADR-007](007-rig-umbrella.md): rig crate selection
- [ADR-008](008-provider-config.md): Config hierarchy

### Tools & Execution
- [ADR-003](003-shell-execution.md): Shell execution model
- [ADR-009](009-tool-set.md): v1 tool set

### v2 Features (Deferred)
- [ADR-005](005-non-streaming.md): Non-streaming v1
- [ADR-010](010-rag-v2.md): RAG pipeline
- [ADR-011](011-ssh-v2.md): SSH integration