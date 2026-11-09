# shai

> **sh**ell **ai** — An AI-powered shell agent that translates natural language to shell commands, executes them with a 4-tier risk classifier, and persists conversation history.

[![Crates.io](https://img.shields.io/crates/v/shai.svg)](https://crates.io/crates/shai)
[![Documentation](https://docs.rs/shai/badge.svg)](https://docs.rs/shai)
[![License](https://img.shields.io/crates/l/shai.svg)](LICENSE)
[![Rust Version](https://img.shields.io/badge/rust-1.75+-blue.svg)](https://www.rust-lang.org)

## Overview

`shai` is a stateful CLI agent that bridges natural language and shell commands. It uses LLMs (via the `rig` framework) to translate your intent into executable commands, evaluates each command through a four-tier risk classifier, prompts for confirmation when needed, and feeds results back to the LLM for continued reasoning.

### Key Features

- **Natural Language → Shell**: Describe what you want in plain English
- **Four-Tier Risk Classification**: Automatic safety evaluation (Allowed → ReadSensitive → WriteRecoverable → Catastrophic)
- **Human-in-the-Loop**: Confirmation prompts for sensitive operations
- **Persistent History**: JSONL conversation history per project (`.shai/session.jsonl`)
- **Multi-Provider Support**: OpenAI, Anthropic, Ollama (local)
- **7 Built-in Tools**: File ops, web search/fetch
- **REPL & One-Shot Modes**: Interactive session or single command

## Installation

### From Crates.io (Recommended)

```bash
cargo install shai
```

### From Source

```bash
git clone https://github.com/yourusername/shai
cd shai
cargo install --path .
```

### Requirements

- Rust 1.75+
- API key for your chosen provider (OpenAI, Anthropic, or Ollama running locally)

## Quick Start

```bash
# Configure (one-time)
export OPENAI_API_KEY="sk-..."
# or
export ANTHROPIC_API_KEY="sk-..."
# or use Ollama locally
export SHAI_PROVIDER="ollama"
export SHAI_MODEL="llama3.1"

# One-shot command
shai "list all Rust files modified this week"

# Interactive REPL
shai
shai> find all TODO comments in the codebase
shai> show me the git diff for the largest file
shai> /exit
```

## Configuration

### Config File: `~/.config/shai/config.toml`

```toml
[provider]
type = "openai"           # "openai" | "anthropic" | "ollama"
model = "gpt-4o"
base_url = ""             # Optional override
api_key = ""              # Optional (env var takes precedence)
temperature = 0.2
max_tokens = 4096

[risk_provider]
type = "openai"           # Optional, defaults to [provider]
model = "gpt-4o-mini"     # Smaller/faster model for risk
temperature = 0.0         # Deterministic classification
max_tokens = 100

[session]
history_path = ".shai/session.jsonl"
max_history_turns = 50

[display]
verbose = false
debug = false
show_reasoning = false
```

### Environment Variables (Override Config)

| Variable | Description |
|----------|-------------|
| `SHAI_PROVIDER` | Provider type |
| `SHAI_MODEL` | Model name |
| `SHAI_API_KEY` | API key |
| `SHAI_TEMPERATURE` | Sampling temperature |
| `SHAI_MAX_TOKENS` | Max tokens per response |
| `SHAI_RISK_PROVIDER` | Risk model provider |
| `SHAI_RISK_MODEL` | Risk model name |
| `SHAI_VERBOSE` | Show full tool output |
| `SHAI_DEBUG` | Show tool calls, risk levels, raw responses |
| `SHAI_SHOW_REASONING` | Show model reasoning (o1, etc.) |
| `OPENAI_API_KEY` | Fallback for OpenAI |
| `ANTHROPIC_API_KEY` | Fallback for Anthropic |
| `OLLAMA_API_BASE_URL` | Fallback for Ollama |

## Usage

### One-Shot Mode

```bash
shai "compress all images in ./assets to webp"
shai -v "run cargo test and summarize failures"
shai --provider anthropic --model claude-3-5-sonnet "explain this codebase"
```

### REPL Mode

```bash
shai                    # Start REPL
shai -v                 # Start REPL in verbose mode
```

**REPL Commands:**

| Command | Description |
|---------|-------------|
| `/exit`, `/quit` | Save history and exit |
| `/history [N]` | Show last N turns (default: 10) |
| `/clear` | Clear history file |
| `/config` | Show current configuration |
| `/risk <cmd>` | Dry-run risk classification |
| `/verbose` | Toggle verbose mode |
| `/debug` | Toggle debug mode |
| `/reasoning` | Toggle reasoning display |
| `/help` | Show help |

### Risk Levels

| Level | Name | Action | Examples |
|-------|------|--------|----------|
| 1 | **Allowed** | Execute immediately | `ls`, `cat file`, `git status` |
| 2 | **ReadSensitive** | Confirm (y/N) | `cat /etc/passwd`, `aws secretsmanager get-secret-value` |
| 3 | **WriteRecoverable** | Confirm (y/N) | `rm file`, `git reset --hard`, `docker rm` |
| 4 | **Catastrophic** | Deny (no prompt) | `rm -rf /`, `dd if=/dev/zero of=/dev/sda` |

## Built-in Tools (7)

| Category | Tools |
|----------|-------|
| **Shell** | `shell` (LLM-classified risk) |
| **File Read** | `read`, `glob`, `grep`, `list_dir` |
| **Web** | `web_search`, `web_fetch` |

## Architecture

```
User Prompt
    ↓
Load History (.shai/session.jsonl)
    ↓
AgentBuilder (Primary Model + 7 Tools)
    ↓
Turn Runner (max_turns=1)
    ↓
LLM Response → Text? → Print & Save
    ↓
Tool Call → Risk Classifier (Risk Model)
    ↓
Level 1 → Execute
Level 2/3 → Prompt User → Execute/Deny
Level 4 → Deny
    ↓
ToolResult → History → Next Turn
```

## Project Structure

```
shai/
├── Cargo.toml
├── src/
│   ├── main.rs           # CLI entry, one-shot vs REPL
│   ├── config.rs         # TOML + env loading, priority merge
│   ├── provider.rs       # Model factory (OpenAI/Anthropic/Ollama)
│   ├── agent.rs          # Turn-by-turn runner
│   ├── history.rs        # JSONL load/save
│   ├── repl.rs           # rustyline REPL
│   ├── risk/
│   │   ├── classifier.rs # Risk evaluation
│   │   └── prompt.rs     # Few-shot prompt template
│   └── tool/
│       ├── mod.rs        # Tool registry & dispatch
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
    ├── architecture/     # Architecture docs
    └── superpowers/      # Design specs & plans
```

## Development

```bash
# Build
cargo build --release

# Test
cargo test

# Run locally
cargo run -- "list files"
cargo run                # REPL
```

## Roadmap

### v1.x (Current - Implemented)
- ✅ Core agent loop with risk classification
- ✅ 7 built-in tools (shell, file ops, web)
- ✅ OpenAI/Anthropic/Ollama providers
- ✅ JSONL history, REPL mode
- ✅ 4-tier risk classifier with user confirmation
- ✅ Configuration via TOML + environment variables
- ✅ Dual LLM architecture (primary + risk model)

### v2.0 (Planned)
- **RAG Pipeline**: Local vector index (`.shai/rag/`) for codebase-aware answers
  - `rag_ingest`: Index files
  - `rag_query`: Semantic search
- **SSH Integration**: Remote command execution
  - `ssh_exec`: Run commands on remote hosts
  - `ssh_copy`: File transfer
  - Key-based auth via `~/.ssh/config`

### v2.x (Future)
- Streaming token output
- Plugin system for custom tools
- GUI/TUI (anti-goal: stay CLI-first)

## Contributing

Contributions are welcome! Please read [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

## Documentation

| Document | Description |
|----------|-------------|
| [Documentation Index](docs/README.md) | Complete documentation index: architecture, specs, quick start, CLI usage, project structure, development |
| [Architecture](docs/architecture/README.md) | Architecture overview, ADRs, diagrams |
| [Specifications](docs/superpowers/specs/) | Design specifications (Superpowers methodology) |
| [Implementation Plans](docs/superpowers/plans/) | Implementation plans with task breakdowns |
| [Testing](docs/testing/README.md) | Testing strategy, organization, templates |
| [Architecture Decision Records](docs/architecture/adr/) | 11 ADRs covering key design decisions |

## License

MIT OR Apache-2.0 — see [LICENSE](LICENSE) for details.

## Acknowledgments

- [rig](https://github.com/rig-rs/rig) — The LLM framework powering shai
- [rustyline](https://github.com/kkawakam/rustyline) — REPL readline support
- [tokio](https://tokio.rs/) — Async runtime