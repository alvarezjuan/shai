# AGENTS.md — shai Development Guide

This document guides AI agents working on the **shai** codebase. It is the single source of truth for how to build, test, and modify this project.

---

## Project Overview

**shai** — AI-powered shell agent that translates natural language to shell commands, executes them via `sh -c` with a 4-tier LLM-based risk classifier, and persists JSONL conversation history per-directory.

- **Language**: Rust 2021
- **Framework**: `rig` 0.46 (features: `agent`, `reqwest`, `derive`)
- **Providers**: OpenAI, Anthropic, Ollama
- **CLI**: `clap` 4 with derive + env
- **REPL**: `rustyline` 12
- **Async**: `tokio` 1 (full features)

---

## Quick Commands

```bash
# Build
cargo build              # debug
cargo build --release    # optimized

# Test
cargo test               # all tests (24 unit tests)
cargo test --lib         # unit tests only

# Check
cargo check              # fast type-check
cargo clippy -- -D warnings  # lints
cargo fmt --check        # formatting check

# Run
cargo run -- "list files"  # one-shot
cargo run                  # REPL mode

# Install
cargo install --path .
```

---

## Architecture

### Core Loop

```
User Prompt
    ↓
Load History (.shai/session.jsonl)
    ↓
AgentBuilder (Primary Model + 15 Tools)
    ↓
Turn Runner (max_turns=1)
    ↓
LLM Response → Text? → Print & Save History
    ↓
Tool Call → Risk Classifier (Risk Model)
    ↓
Level 1 (Allowed) → Execute immediately
Level 2/3 (ReadSensitive/WriteRecoverable) → Prompt user y/N
Level 4 (Catastrophic) → Deny
    ↓
ToolResult → History → Next Turn
```

### Key Design Decisions (from ADRs)

| ADR | Decision |
|-----|----------|
| ADR-001 | LLM-only risk classification (no hardcoded deny/allow) |
| ADR-002 | Manual turn-by-turn loop with `max_turns(1)` |
| ADR-003 | Shell execution via `sh -c` |
| ADR-004 | Per-directory history: `.shai/session.jsonl` |
| ADR-005 | Non-streaming for v1 |
| ADR-006 | Dual LLM: primary + separate risk model |
| ADR-007 | rig 0.46 umbrella crate with `agent` feature |
| ADR-008 | Config: TOML + env vars (Env > TOML > Defaults) |
| ADR-009 | 15 v1 tools (read/write/inspect/web/git) |
| ADR-010 | RAG v2 deferred |
| ADR-011 | SSH v2 deferred |

---

## Project Structure

```
shai/
├── Cargo.toml
├── src/
│   ├── main.rs           # CLI entry: one-shot vs REPL
│   ├── config.rs         # TOML + env loading, priority merge
│   ├── provider.rs       # Model factory (OpenAI/Anthropic/Ollama)
│   ├── agent.rs          # Turn-by-turn runner
│   ├── history.rs        # JSONL load/save
│   ├── repl.rs           # rustyline REPL
│   ├── risk/
│   │   ├── classifier.rs # Risk evaluation
│   │   └── prompt.rs     # Few-shot risk prompt
│   └── tool/
│       ├── mod.rs        # Tool registry & dispatch
│       ├── shell.rs      # Shell execution with risk
│       ├── read.rs       # File reading
│       ├── glob.rs       # Pattern matching
│       ├── grep.rs       # Content search
│       ├── list_dir.rs   # Directory listing
│       ├── web_search.rs # DuckDuckGo scraping
│       ├── web_fetch.rs  # URL content extraction
│       ├── git.rs        # Git operations (implemented, stubbed for git2/openssl)
│       ├── edit.rs       # File editing with diff
│       └── todo.rs       # In-memory task list
├── tests/
│   └── integration.rs
└── docs/
    ├── architecture/     # ADRs, component, data-flow, config, etc.
    └── superpowers/      # Specs & plans
```

---

## Key Interfaces

### Config (`src/config.rs`)

```rust
Config::load() -> Result<Config>
// Priority: Env > ~/.config/shai/config.toml > .shai/config.toml > Defaults

// Key structs: Config, ProviderConfig, RiskProviderConfig, SessionConfig, DisplayConfig
```

### Provider Factory (`src/provider.rs`)

```rust
build_model(&ProviderConfig) -> Result<DynModel<Completion>>
// Supports: openai, anthropic, ollama
// Uses OpenAIConfig::client(), AnthropicConfig::client(), Ollama::new()
```

### Agent Loop (`src/agent.rs`)

```rust
run_agent(prompt: &str, config: &Config) -> Result<String>
// Builds AgentBuilder with all 15 tools
// Manual loop: max_turns(1), handles tool calls, risk, confirmation
```

### Risk Classifier (`src/risk/classifier.rs`)

```rust
RiskClassifier::new(model: DynModel<Completion>)
evaluate(command: &str, cwd: &str, recent: &[String]) -> Result<RiskResult>
// RiskResult { level: RiskLevel, reason: String }
// RiskLevel: Allowed(1), ReadSensitive(2), WriteRecoverable(3), Catastrophic(4)
```

### Tool Registry (`src/tool/mod.rs`)

```rust
ToolRegistry::dispatch(tool_call: &ToolCall, risk_classifier: &RiskClassifier, config: &Config) -> Result<ToolResult>
// Handles 15 tools, integrates risk for shell, confirm for write tools
```

---

## Tools (15 v1)

| Tool | Category | Risk | Confirmation |
|------|----------|------|--------------|
| `shell` | Execute | LLM (1-4) | L2/3: y/N, L4: deny |
| `read`, `glob`, `grep`, `list_dir` | File Read | 1 (Auto) | None |
| `web_search`, `web_fetch` | Web | 1 (Auto) | None |
| `git_status`, `git_diff`, `git_log` | Git Read | 1 (Auto) | None |
| `git_commit`, `git_push` | Git Write | 3 (Confirm) | Yes |
| `edit`, `patch` | File Write | 3 (Confirm) | Yes |
| `todo_read`, `todo_write` | Tasks | 1 (Auto) | None |

---

## Testing

```bash
# All tests
cargo test

# Unit tests only
cargo test --lib

# Specific test
cargo test tool::shell::tests::test_execute_simple_command

# With output
cargo test -- --nocapture
```

### Test Conventions

- Unit tests in `#[cfg(test)]` modules alongside code
- Use `tempfile` for filesystem tests
- Test within CWD (path validation requires it)
- No integration tests requiring API keys in CI

---

## Common Tasks

### Add a New Tool

1. Create `src/tool/your_tool.rs` implementing `rig::tool::Tool`
2. Add `description()`, `parameters()` (via `schemars::schema_for!`)
3. Register in `src/tool/mod.rs` dispatch
4. Add risk handling in `handle_*`
5. Write unit tests in `#[cfg(test)]` module
6. Update `README.md` tool table

### Risk Levels for New Tools

| Level | When |
|-------|------|
| 1 (Allowed) | Read-only, no side effects, no sensitive data |
| 2 (ReadSensitive) | Reads secrets, PII, auth tokens |
| 3 (WriteRecoverable) | Writes/deletes but recoverable |
| 4 (Catastrophic) | Irrecoverable destruction |

### Modify Risk Prompt

Edit `src/risk/prompt.rs`:
- `build_risk_prompt()` — template with few-shot examples
- JSON output: `{"level": 1-4, "reason": "..."}`

---

## Config System

### Priority (High → Low)

1. Environment variables (`SHAI_*`, `OPENAI_API_KEY`, etc.)
2. `~/.config/shai/config.toml`
3. `.shai/config.toml` (per-project, future)
4. Built-in defaults

### Key Env Vars

| Variable | Maps To |
|----------|---------|
| `SHAI_PROVIDER` | `provider.type` |
| `SHAI_MODEL` | `provider.model` |
| `SHAI_API_KEY` | `provider.api_key` |
| `SHAI_RISK_PROVIDER` | `risk_provider.type` |
| `SHAI_RISK_MODEL` | `risk_provider.model` |
| `SHAI_VERBOSE` | `display.verbose` |
| `SHAI_DEBUG` | `display.debug` |
| `OPENAI_API_KEY` | Fallback for OpenAI |
| `ANTHROPIC_API_KEY` | Fallback for Anthropic |

---

## Git Workflow

```bash
# Feature branch
git checkout -b feat/my-feature

# Pre-commit checks
cargo fmt --check && cargo clippy -- -D warnings && cargo test

# Commit (Conventional Commits)
git commit -m "feat: add web_search tool"

# Push and PR
git push origin feat/my-feature
```

---

## Documentation

- `README.md` — User-facing docs
- `CONTRIBUTING.md` — Contribution guide
- `CHANGELOG.md` — Keep a Changelog format
- `SECURITY.md` — Vulnerability reporting
- `docs/architecture/` — ADRs, diagrams
- `docs/superpowers/specs/` — Design specs
- `docs/superpowers/plans/` — Implementation plans

---

## Gotchas

1. **Path validation**: File tools require paths within CWD. Tests must create temp dirs within CWD.
2. **Risk classifier**: Uses separate LLM call; may be non-deterministic. Tests use fixed JSON.
3. **Shell execution**: Uses `sh -c` — supports pipes, redirects, builtins.
4. **History**: JSONL, one `Message` per line. Max 50 turns loaded.
5. **Git tools**: Currently stubbed (need `git2` with openssl).
6. **Dual LLM**: Risk model falls back to primary if not configured.
7. **Provider fallbacks**: `OPENAI_API_KEY`, `ANTHROPIC_API_KEY`, `OLLAMA_API_BASE_URL`.

---

## References

- `docs/architecture/adr/` — 11 Architecture Decision Records
- `docs/superpowers/specs/2026-11-08-ai-shell-agent-design.md` — Full design spec
- `config.example.toml` — Config template
- `rig` docs: https://github.com/rig-rs/rig

---

## Documentation Index

| Document | Path | Description |
|----------|------|-------------|
| **Documentation Index** | [docs/README.md](docs/README.md) | Complete documentation index: architecture, specs, quick start, CLI usage, project structure, development |
| **Architecture Overview** | [docs/architecture/README.md](docs/architecture/README.md) | Architecture overview, ADRs, diagrams |
| **Architecture Decision Records** | [docs/architecture/adr/](docs/architecture/adr/) | 11 ADRs covering key design decisions |
| **Specifications** | [docs/superpowers/specs/](docs/superpowers/specs/) | Design specifications (Superpowers methodology) |
| **Implementation Plans** | [docs/superpowers/plans/](docs/superpowers/plans/) | Implementation plans with task breakdowns |
| **Testing Documentation** | [docs/testing/README.md](docs/testing/README.md) | Testing strategy, organization, templates |
| **Architecture Decision Records** | [docs/architecture/adr/](docs/architecture/adr/) | 11 ADRs covering key design decisions |
| **Main Project README** | [README.md](README.md) | User-facing docs, quick start, usage |

---

*This document is the single source of truth for agent behavior on this codebase. Keep it current.*