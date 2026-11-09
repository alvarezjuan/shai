# shai Documentation

**shai** — AI-powered shell agent with natural-language to shell command translation, 4-tier risk classifier, and persistent conversation history.

---

## Documentation Index

### Architecture

| Document | Description |
|----------|-------------|
| [architecture/README.md](architecture/README.md) | Architecture overview with quick reference |
| [architecture/adr/README.md](architecture/adr/README.md) | 11 Architecture Decision Records (ADR-001 through ADR-011) |
| [architecture/component.md](architecture/component.md) | Component architecture + responsibilities |
| [architecture/data-flow.md](architecture/data-flow.md) | One-turn data flow with risk handling |
| [architecture/config.md](architecture/config.md) | Configuration hierarchy, schema, env vars |
| [architecture/project-structure.md](architecture/project-structure.md) | Source tree, modules, dependencies |
| [architecture/dependencies.md](architecture/dependencies.md) | Cargo.toml dependencies (v1 + v2) |
| [architecture/sequence.md](architecture/sequence.md) | One-shot vs REPL sequence diagrams |
| [architecture/v1-v2.md](architecture/v1-v2.md) | Version scope comparison |

### Specifications

| Document | Description |
|----------|-------------|
| [superpowers/specs/2026-11-08-ai-shell-agent-design.md](superpowers/specs/2026-11-08-ai-shell-agent-design.md) | Complete design specification |

---

## Quick Start

### Core Concepts

- **Agent Loop**: Manual turn-by-turn with `max_turns(1)` — full control over risk/confirm/exec
- **Dual LLM**: Primary (command generation) + Risk (classification) — separate configs
- **Risk Levels**: Allowed (1) → ReadSensitive (2) → WriteRecoverable (3) → Catastrophic (4)
- **History**: JSONL in `.shai/session.jsonl` (per-directory, 50 turns max)
- **Config**: TOML + env vars (Env > TOML > Defaults)

### v1 Tool Set (15 tools)

| Category | Tools | Risk | Confirmation |
|----------|-------|------|--------------|
| Shell | `shell` | 1–4 (LLM) | Level 2/3: y/N, Level 4: deny |
| File Read | `read`, `glob`, `grep`, `list_dir` | 1 | None |
| Web | `web_search`, `web_fetch` | 1 | None |
| Git Read | `git_status`, `git_diff`, `git_log` | 1 | None |
| Git Write | `git_commit`, `git_push` | 3 | Yes |
| File Write | `edit`, `patch` | 3 | Yes |
| Tasks | `todo_read`, `todo_write` | 1 | None |

### v2 (Deferred)

- **RAG**: `rag_query`, `rag_ingest` — local vector index in `.shai/rag/`
- **SSH**: `ssh_exec`, `ssh_copy` — key-based, uses `~/.ssh/config`
- **Streaming**: Token-by-token output
- **Plugins**: Custom tool loading

---

## Tech Stack

| Component | Technology |
|-----------|------------|
| Language | Rust 2021 |
| LLM Framework | `rig` 0.46 (features: `agent`, `reqwest`, `derive`) |
| Providers | OpenAI, Anthropic, Ollama |
| CLI | `clap` 4 (derive + env) |
| REPL | `rustyline` 12 |
| Async | `tokio` 1 (full) |
| Serialization | `serde`, `serde_json`, `schemars` |

---

## Configuration

```toml
# ~/.config/shai/config.toml
[provider]
type = "openai"        # openai | anthropic | ollama
model = "gpt-4o"
temperature = 0.2
max_tokens = 4096

[risk_provider]
type = "openai"        # optional, defaults to [provider]
model = "gpt-4o-mini"
temperature = 0.0
max_tokens = 100

[session]
history_path = ".shai/session.jsonl"
max_history_turns = 50
```

Environment variables (take precedence):
- Primary: `SHAI_PROVIDER`, `SHAI_MODEL`, `SHAI_API_KEY`, `SHAI_TEMPERATURE`, `SHAI_MAX_TOKENS`
- Risk: `SHAI_RISK_PROVIDER`, `SHAI_RISK_MODEL`, `SHAI_RISK_API_KEY`, `SHAI_RISK_TEMPERATURE`, `SHAI_RISK_MAX_TOKENS`
- Provider fallbacks: `OPENAI_API_KEY`, `ANTHROPIC_API_KEY`, `OLLAMA_API_BASE_URL`

---

## CLI Usage

```bash
# One-shot
shai "list all .rs files modified this week"

# REPL
shai
shai> list all .rs files modified this week
shai> now show me the diff for the largest one
shai> /exit

# Flags (one-shot or REPL)
shai -v "prompt"      # verbose: show tool output
shai -d "prompt"      # debug: show tool calls, risk levels, raw LLM responses
shai -r "prompt"      # reasoning: show model reasoning (if supported)
```

### REPL Commands

| Command | Action |
|---------|--------|
| `/exit`, `/quit` | Save history, exit |
| `/history [N]` | Show last N turns |
| `/clear` | Clear history file |
| `/config` | Show current config |
| `/risk <cmd>` | Dry-run risk classification |
| `/verbose` | Toggle verbose flag |
| `/debug` | Toggle debug flag |
| `/reasoning` | Toggle reasoning flag |

---

## Project Structure

```
shai/
├── Cargo.toml
├── src/
│   ├── main.rs           # CLI entry: one-shot vs REPL
│   ├── config.rs         # TOML + env loading, priority merge
│   ├── provider.rs       # Factory → DynModel<Completion>
│   ├── agent.rs          # AgentBuilder, turn-by-turn runner
│   ├── history.rs        # JSONL load/save, Message serde
│   ├── repl.rs           # rustyline REPL, /commands
│   ├── risk/
│   │   ├── classifier.rs # RiskClassifier::evaluate()
│   │   └── prompt.rs     # Few-shot prompt, JSON parsing
│   ├── tool/
│   │   ├── mod.rs        # Registry, dispatch
│   │   ├── shell.rs      # shell tool (LLM risk)
│   │   ├── read.rs       # read tool
│   │   ├── glob.rs       # glob tool
│   │   ├── grep.rs       # grep tool
│   │   ├── list_dir.rs   # list_dir tool
│   │   ├── web_search.rs # web_search tool
│   │   ├── web_fetch.rs  # web_fetch tool
│   │   ├── git.rs        # git_status/diff/log/commit/push
│   │   ├── edit.rs       # edit, patch tools
│   │   └── todo.rs       # todo_read/write tools
│   ├── rag/              # v2: mod.rs, index.rs, embed.rs
│   └── ssh/              # v2: mod.rs, config.rs
├── tests/
│   └── integration.rs
└── docs/
    ├── architecture/     # This documentation
    └── superpowers/      # Design specs
```

---

## Development

### Building

```bash
cargo build --release
```

### Testing

```bash
cargo test
```

### Running Locally

```bash
cargo run -- "list files"
cargo run -- --verbose "git status"
cargo run  # REPL mode
```

---

## Related

- **Main Repo**: [github.com/...](https://github.com/...) (add your repo URL)
- **rig Documentation**: [rig.rs](https://rig.rs/)
- **Issue Tracker**: GitHub Issues