---
layout: default
title: shai
---

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

## Documentation

| Section | Description |
|---------|-------------|
| [Quick Start](/quick-start/) | Get up and running in minutes |
| [Configuration](/configuration/) | Config file, environment variables, providers |
| [Usage](/usage/) | One-shot, REPL, risk levels, examples |
| [Architecture](/architecture/) | Core loop, ADRs, data flow, components |
| [Tools](/tools/) | Built-in tools reference |
| [Development](/development/) | Building, testing, contributing |

## Installation

### From Crates.io (Recommended)

```bash
cargo install shai
```

### From Source

```bash
git clone https://github.com/alvarezjuan/shai
cd shai
cargo install --path .
```

### Requirements

- Rust 1.75+
- API key for your chosen provider (OpenAI, Anthropic, or Ollama running locally)

## Links

- **GitHub**: [https://github.com/alvarezjuan/shai](https://github.com/alvarezjuan/shai)
- **Crates.io**: [https://crates.io/crates/shai](https://crates.io/crates/shai)
- **Docs.rs**: [https://docs.rs/shai](https://docs.rs/shai)
- **Issues**: [https://github.com/alvarezjuan/shai/issues](https://github.com/alvarezjuan/shai/issues)

## License

MIT OR Apache-2.0 — see [LICENSE](LICENSE) for details.
