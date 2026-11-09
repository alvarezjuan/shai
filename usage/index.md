---
layout: default
title: Usage
nav_order: 3
---

# Usage

## Command Line Interface

```bash
shai [OPTIONS] [PROMPT]
```

### Global Options

| Option | Short | Description |
|--------|-------|-------------|
| `--verbose` | `-v` | Show full tool output (stdout, stderr, exit code) |
| `--debug` | `-d` | Show tool calls, risk levels, raw LLM responses, timing |
| `--show-reasoning` | | Show model reasoning (if supported: o1, deepseek-r1, etc.) |
| `--provider` | | Override provider type (openai/anthropic/ollama) |
| `--model` | | Override model name |
| `--help` | `-h` | Show help |
| `--version` | `-V` | Show version |

### Subcommands

```bash
shai config show          # Display current config
shai config set KEY VALUE # Set config value (not yet implemented)
shai config init          # Initialize config file (not yet implemented)
```

## One-Shot Mode

Execute a single prompt and exit:

```bash
# Basic usage
shai "list all Rust files"

# With flags
shai -v "run cargo test and summarize failures"
shai --provider anthropic --model claude-3-5-sonnet "explain this codebase"

# Pipe input
echo "find all TODO comments" | shai
```

### Exit Codes

| Code | Meaning |
|------|---------|
| 0 | Success |
| 1 | General error |
| 2 | Risk level 4 (Catastrophic) — command denied |
| 3 | User denied confirmation |
| 130 | Interrupted (Ctrl+C) |

## REPL Mode

Start an interactive session:

```bash
shai              # Start REPL
shai -v           # Start with verbose
shai --debug      # Start with debug
```

### REPL Commands

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

### REPL Example

```bash
$ shai
shai> list files in src/
# Executes: ls -la src/
shai> show me main.rs
# Executes: cat src/main.rs
shai> run tests
# Executes: cargo test
shai> /history 5
# Shows last 5 turns
shai> /risk "rm -rf /"
# Risk level: 4 (Catastrophic) - denied
shai> /exit
```

## Risk Levels

Every shell command is classified by the risk model:

| Level | Name | Action | Examples |
|-------|------|--------|----------|
| 1 | **Allowed** | Execute immediately | `ls`, `cat file`, `git status`, `cargo build` |
| 2 | **ReadSensitive** | Confirm (y/N) | `cat /etc/passwd`, `aws secretsmanager get-secret-value`, `env` |
| 3 | **WriteRecoverable** | Confirm (y/N) | `rm file`, `git reset --hard`, `docker rm`, `cargo clean` |
| 4 | **Catastrophic** | Deny (no prompt) | `rm -rf /`, `dd if=/dev/zero of=/dev/sda`, `:(){ :|:& };:` |

### Risk Confirmation Flow

```
User: shai "delete all .tmp files"
    ↓
Risk Classifier evaluates: "find . -name '*.tmp' -delete"
    ↓
Level 3 (WriteRecoverable) → Prompts: "Execute? [y/N]"
    ↓
User: y → Executes command
User: n → Cancels, returns to prompt
```

## Built-in Tools (7)

| Category | Tool | Description |
|----------|------|-------------|
| **Execute** | `shell` | Run shell commands (risk-classified) |
| **File Read** | `read` | Read file contents |
| | `glob` | Find files by pattern |
| | `grep` | Search file contents |
| | `list_dir` | List directory contents |
| **Web** | `web_search` | Search DuckDuckGo |
| | `web_fetch` | Fetch URL content |

### Tool Usage Examples

```bash
# Shell (auto-risk-classified)
shai "run cargo test"

# File operations (auto-allowed)
shai "show me Cargo.toml"
shai "find all .rs files"
shai "search for 'TODO' in src/"
shai "list src/ directory"

# Web (auto-allowed)
shai "search for Rust async best practices"
shai "fetch https://doc.rust-lang.org/book/"
```

## History

- **Location**: `.shai/session.jsonl` (per project)
- **Format**: JSONL, one `Message` per line
- **Retention**: Last 50 turns loaded by default
- **Commands**: `/history`, `/clear`

```bash
# View history file
cat .shai/session.jsonl

# In REPL
shai> /history 20
shai> /clear
```

## Examples

### File Operations

```bash
shai "find all test files"
shai "show me the largest file in src/"
shai "search for 'unwrap()' in the codebase"
shai "read the README.md"
```

### Git Operations

```bash
shai "show git status"
shai "show diff for last commit"
shai "list last 10 commits"
```

### Development Tasks

```bash
shai "run cargo test and show failures"
shai "run cargo clippy and show warnings"
shai "build release binary"
shai "check for outdated dependencies"
```

### Web Search

```bash
shai "search for Rust 2024 edition features"
shai "fetch https://github.com/rust-lang/rust"
```

### System Operations

```bash
shai "show disk usage"
shai "list running processes"
shai "show environment variables"
```
