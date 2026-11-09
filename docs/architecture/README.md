# Architecture Documentation

This folder contains the architectural decisions and design documents for **shai** — an AI-powered shell agent.

## Contents

| File | Description |
|------|-------------|
| [adr/README.md](adr/README.md) | **Architecture Decision Records** — 11 individual files (001-011) |
| [component.md](component.md) | Component architecture diagram + responsibilities |
| [data-flow.md](data-flow.md) | One-turn data flow with phases and tool risk levels |
| [config.md](config.md) | Configuration hierarchy, full schema, env vars |
| [project-structure.md](project-structure.md) | Source tree, module responsibilities, dependencies |
| [dependencies.md](dependencies.md) | Cargo.toml dependencies by category (v1 + v2) |
| [sequence.md](sequence.md) | Sequence diagrams: One-shot vs REPL |
| [v1-v2.md](v1-v2.md) | Version scope comparison and migration path |

## Quick Reference

### Core Architecture
- **Agent Loop**: Manual turn-by-turn with `max_turns(1)` (Option B) — [ADR-002](adr/002-manual-agent-loop.md)
- **Risk Classification**: Dual LLM — primary generates commands, separate risk model classifies — [ADR-006](adr/006-dual-llm-risk.md)
- **Shell Execution**: `sh -c` for pipes/redirects/builtins — [ADR-003](adr/003-shell-execution.md)
- **History**: JSONL in `.shai/session.jsonl` (per-directory) — [ADR-004](adr/004-per-directory-history.md)

### Configuration
- **Primary Model**: `[provider]` — OpenAI/Anthropic/Ollama
- **Risk Model**: `[risk_provider]` — separate config, defaults to provider
- **Display**: `[display]` — verbose, debug, show_reasoning flags
- **RAG (v2)**: `[rag]` — local vector index in `.shai/rag/`
- **SSH (v2)**: `[ssh]` — key-based, uses `~/.ssh/config`

### Tool Set

| v1 (15 tools) | v2 (Deferred) |
|---------------|---------------|
| `shell` | `rag_query`, `rag_ingest` |
| `read`, `glob`, `grep`, `list_dir` | `ssh_exec`, `ssh_copy` |
| `web_search`, `web_fetch` | |
| `git_status`, `git_diff`, `git_log` | |
| `git_commit`, `git_push` | |
| `edit`, `patch` | |
| `todo_read`, `todo_write` | |

### Risk Levels (for `shell` tool)
| Level | Name | Action |
|-------|------|--------|
| 1 | Allowed | Execute immediately |
| 2 | ReadSensitive | Confirm (y/N) |
| 3 | WriteRecoverable | Confirm (y/N) |
| 4 | Catastrophic | Deny (no prompt) |

### Tech Stack
- **Language**: Rust 2021
- **LLM Framework**: `rig` 0.46 (umbrella crate) with features `["agent", "reqwest", "derive"]`
- **Providers**: OpenAI, Anthropic, Ollama
- **CLI**: `clap` with derive + env
- **REPL**: `rustyline`
- **Async**: `tokio` (full features)
- **Serialization**: `serde`, `serde_json`, `schemars`

## Related Documents

- **Main Design Spec**: `../superpowers/specs/2026-11-08-ai-shell-agent-design.md`
- **Implementation Plan**: (to be created via `writing-plans`)

## Diagrams

All diagrams are embedded as Mermaid blocks directly in their respective `.md` files (component.md, data-flow.md, config.md, project-structure.md, dependencies.md, sequence.md, v1-v2.md). Render in:
- **GitHub/GitLab**: Native Mermaid support
- **VS Code**: Install "Markdown Preview Mermaid Support" extension
- **Obsidian**: Native Mermaid support
- **CLI**: `mmdc -i file.mmd -o file.svg` (requires `@mermaid-js/mermaid-cli`)

---

## File Layout

```
docs/architecture/
├── README.md              # Architecture index
├── adr/                   # 11 Architecture Decision Records
│   ├── 001-llm-only-risk.md
│   ├── 002-manual-agent-loop.md
│   ├── 003-shell-execution.md
│   ├── 004-per-directory-history.md
│   ├── 005-non-streaming.md
│   ├── 006-dual-llm-risk.md
│   ├── 007-rig-umbrella.md
│   ├── 008-provider-config.md
│   ├── 009-tool-set.md
│   ├── 010-rag-v2.md
│   └── 011-ssh-v2.md
├── component.md           # Component architecture + Mermaid diagram
├── data-flow.md           # One-turn data flow + Mermaid diagram
├── config.md              # Configuration hierarchy + Mermaid diagram
├── project-structure.md   # Source tree + Mermaid diagram
├── dependencies.md        # Dependencies + Mermaid diagram
├── sequence.md            # Sequence diagrams + Mermaid diagram
└── v1-v2.md               # Version scope + Mermaid diagram
```

---

## Related

- **Main Design Spec**: `../superpowers/specs/2026-11-08-ai-shell-agent-design.md`
- **ADRs**: `adr/` (11 decisions)