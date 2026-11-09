---
layout: default
title: Quick Start
nav_order: 1
---

# Quick Start

Get up and running with shai in minutes.

## Prerequisites

- **Rust 1.75+** — Install from [rustup.rs](https://rustup.rs/)
- **API Key** — For OpenAI, Anthropic, or run Ollama locally

## Installation

### Option 1: From Crates.io (Recommended)

```bash
cargo install shai
```

### Option 2: From Source

```bash
git clone https://github.com/alvarezjuan/shai
cd shai
cargo install --path .
```

## Configuration

### Quick Setup (Environment Variables)

```bash
# OpenAI
export OPENAI_API_KEY="sk-..."

# Anthropic
export ANTHROPIC_API_KEY="sk-..."

# Ollama (local)
export SHAI_PROVIDER="ollama"
export SHAI_MODEL="llama3.1"
```

### Persistent Config File

```bash
mkdir -p ~/.config/shai
cat > ~/.config/shai/config.toml << 'TOML'
[provider]
type = "openai"
model = "gpt-4o"
temperature = 0.2
max_tokens = 4096

[risk_provider]
type = "openai"
model = "gpt-4o-mini"
temperature = 0.0
max_tokens = 100

[session]
history_path = ".shai/session.jsonl"
max_history_turns = 50

[display]
verbose = false
debug = false
show_reasoning = false
TOML
```

## First Commands

### One-Shot Mode

```bash
# List files
shai "list all Rust files in the project"

# Run tests and summarize
shai "run cargo test and show me failures"

# Search code
shai "find all TODO comments in src/"
```

### REPL Mode

```bash
# Start interactive session
shai

# With verbose output
shai -v
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

## Example Session

```bash
$ shai
shai> list all files in the current directory
# shai executes: ls -la
shai> show me the Cargo.toml
# shai executes: cat Cargo.toml
shai> run tests and show summary
# shai executes: cargo test
shai> /exit
```

## Next Steps

- [Configuration](/configuration/) — Detailed config options
- [Usage](/usage/) — One-shot, REPL, risk levels, examples
- [Architecture](/architecture/) — How shai works internally
