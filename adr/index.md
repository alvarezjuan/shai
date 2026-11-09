---
layout: default
title: Architecture Decision Records
nav_order: 7
parent: Architecture
---

# Architecture Decision Records (ADRs)

| ADR | Title | Status |
|-----|-------|--------|
| [001](001-llm-only-risk.md) | LLM-Only Risk Classification | Accepted |
| [002](002-manual-agent-loop.md) | Manual Agent Loop | Accepted |
| [003](003-shell-execution.md) | Shell Execution via sh -c | Accepted |
| [004](004-per-directory-history.md) | Per-Directory History | Accepted |
| [005](005-non-streaming.md) | Non-Streaming v1 | Accepted |
| [006](006-dual-llm-risk.md) | Dual LLM: Primary + Risk Model | Accepted |
| [007](007-rig-umbrella.md) | Rig Umbrella Crate | Accepted |
| [008](008-provider-config.md) | Config: TOML + Env Vars | Accepted |
| [009](009-tool-set.md) | 7 v1 Tools | Accepted |
| [010](010-rag-v2.md) | RAG v2 Deferred | Accepted |
| [011](011-ssh-v2.md) | SSH v2 Deferred | Accepted |

## ADR Format

Each ADR follows this structure:

```markdown
# ADR-XXX: Title

## Status
Accepted / Superseded / Deprecated

## Context
What is the issue that we're seeing that is motivating this decision?

## Decision
What is the change that we're proposing and/or doing?

## Consequences
What becomes easier or more difficult to do because of this change?
```

## Contributing

To propose a new ADR:
1. Copy `template.md` to `XXX-title.md`
2. Fill in the sections
3. Submit PR for review
