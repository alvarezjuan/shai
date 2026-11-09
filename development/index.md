---
layout: default
title: Development
nav_order: 6
---

# Development

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

## Project Structure

```
shai/
├── Cargo.toml
├── src/
│   ├── main.rs           # CLI entry
│   ├── config.rs         # TOML + env loading
│   ├── provider.rs       # Model factory
│   ├── agent.rs          # Turn-by-turn runner
│   ├── history.rs        # JSONL load/save
│   ├── repl.rs           # rustyline REPL
│   ├── risk/
│   │   ├── classifier.rs # Risk evaluation
│   │   └── prompt.rs     # Few-shot prompt
│   └── tool/
│       ├── mod.rs        # Registry & dispatch
│       ├── shell.rs      # Shell execution
│       ├── read.rs       # File reading
│       ├── glob.rs       # Pattern matching
│       ├── grep.rs       # Content search
│       ├── list_dir.rs   # Directory listing
│       ├── web_search.rs # Web search
│       └── web_fetch.rs  # URL fetching
├── tests/
│   └── integration.rs
└── docs/
    ├── architecture/
    └── superpowers/
```

## Adding a New Tool

1. Create `src/tool/your_tool.rs` implementing `rig::tool::Tool`
2. Add `description()`, `parameters()` (via `schemars::schema_for!`)
3. Register in `src/tool/mod.rs` dispatch
4. Add risk handling in `handle_*`
5. Write unit tests in `#[cfg(test)]` module

## Risk Levels for New Tools

| Level | When |
|-------|------|
| 1 (Allowed) | Read-only, no side effects, no sensitive data |
| 2 (ReadSensitive) | Reads secrets, PII, auth tokens |
| 3 (WriteRecoverable) | Writes/deletes but recoverable |
| 4 (Catastrophic) | Irrecoverable destruction |

## Modifying Risk Prompt

Edit `src/risk/prompt.rs`:
- `build_risk_prompt()` — template with few-shot examples
- JSON output: `{"level": 1-4, "reason": "..."}`

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

## Documentation

- `README.md` — User-facing docs
- `CONTRIBUTING.md` — Contribution guide
- `CHANGELOG.md` — Keep a Changelog format
- `SECURITY.md` — Vulnerability reporting
- `docs/architecture/` — ADRs, diagrams
- `docs/superpowers/specs/` — Design specs
- `docs/superpowers/plans/` — Implementation plans

## Dependencies

Key dependencies (see `Cargo.toml`):

| Crate | Purpose |
|-------|---------|
| `rig` | LLM framework (0.46, features: agent, reqwest, derive) |
| `clap` | CLI parsing (4, derive + env) |
| `rustyline` | REPL (12) |
| `tokio` | Async runtime (1, full) |
| `serde` | Serialization |
| `toml` | Config parsing |
| `schemars` | JSON Schema for tools |
| `anyhow` | Error handling |
| `tempfile` | Testing |

## CI/CD

GitHub Actions (`.github/workflows/`):
- Build and test on push/PR
- Clippy and fmt checks
- Release on tag push

## Debugging

```bash
# Debug mode
shai --debug "test command"

# Verbose mode
shai -v "test command"

# Show reasoning (o1, deepseek-r1)
shai --show-reasoning "complex task"

# Dry-run risk classification
shai> /risk "rm -rf /"
```

## Gotchas

1. **Path validation**: File tools require paths within CWD. Tests must create temp dirs within CWD.
2. **Risk classifier**: Uses separate LLM call; may be non-deterministic. Tests use fixed JSON.
3. **Shell execution**: Uses `sh -c` — supports pipes, redirects, builtins.
4. **History**: JSONL, one `Message` per line. Max 50 turns loaded.
5. **Dual LLM**: Risk model falls back to primary if not configured.
6. **Provider fallbacks**: `OPENAI_API_KEY`, `ANTHROPIC_API_KEY`, `OLLAMA_API_BASE_URL`.

## References

- `docs/architecture/adr/` — 11 Architecture Decision Records
- `docs/superpowers/specs/2026-11-08-ai-shell-agent-design.md` — Full design spec
- `config.example.toml` — Config template
- `rig` docs: https://github.com/rig-rs/rig
