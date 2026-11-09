---
layout: default
title: Configuration
nav_order: 2
---

# Configuration

shai uses a layered configuration system with the following priority (highest to lowest):

1. **Environment Variables** — `SHAI_*`, `OPENAI_API_KEY`, etc.
2. **Config File** — `~/.config/shai/config.toml`
3. **Built-in Defaults**

## Config File Location

```
~/.config/shai/config.toml
```

## Full Config Example

```toml
[provider]
# Provider type: "openai" | "anthropic" | "ollama"
type = "openai"

# Model name (provider-specific)
model = "gpt-4o"

# Optional: Custom base URL (e.g., for OpenAI-compatible proxies)
# base_url = "https://api.openai.com/v1"

# Optional: API key (can also use OPENAI_API_KEY env var)
# api_key = "sk-..."

# Temperature for command generation (0.0-2.0)
temperature = 0.2

# Max tokens per response
max_tokens = 4096

[risk_provider]
# Optional: Separate model for risk classification
# If omitted, falls back to [provider] with risk-appropriate defaults
type = "openai"
model = "gpt-4o-mini"
# base_url = ""
# api_key = ""

# Deterministic classification
temperature = 0.0
max_tokens = 100

[session]
# History file path (relative to cwd or absolute)
history_path = ".shai/session.jsonl"

# Maximum conversation turns to keep in context
max_history_turns = 50

[display]
# Show full tool output (stdout, stderr, exit code)
verbose = false

# Show tool calls, risk levels, raw LLM responses, timing
debug = false

# Show model reasoning (if supported: o1, deepseek-r1, etc.)
show_reasoning = false

# === v2 Features (disabled by default) ===

#[rag]
#enabled = false
#index_path = ".shai/rag"
#embedding_model = "nomic-embed-text"
#chunk_size = 512
#chunk_overlap = 50

#[ssh]
#enabled = false
#known_hosts = []
```

## Environment Variables

All config options can be overridden via environment variables:

| Variable | Config Path | Description |
|----------|-------------|-------------|
| `SHAI_PROVIDER` | `provider.type` | Provider type (openai/anthropic/ollama) |
| `SHAI_MODEL` | `provider.model` | Model name |
| `SHAI_API_KEY` | `provider.api_key` | API key |
| `SHAI_TEMPERATURE` | `provider.temperature` | Sampling temperature |
| `SHAI_MAX_TOKENS` | `provider.max_tokens` | Max tokens per response |
| `SHAI_BASE_URL` | `provider.base_url` | Custom API base URL |
| `SHAI_RISK_PROVIDER` | `risk_provider.type` | Risk model provider |
| `SHAI_RISK_MODEL` | `risk_provider.model` | Risk model name |
| `SHAI_RISK_TEMPERATURE` | `risk_provider.temperature` | Risk model temperature |
| `SHAI_RISK_MAX_TOKENS` | `risk_provider.max_tokens` | Risk model max tokens |
| `SHAI_VERBOSE` | `display.verbose` | Show full tool output |
| `SHAI_DEBUG` | `display.debug` | Show debug info |
| `SHAI_SHOW_REASONING` | `display.show_reasoning` | Show model reasoning |
| `SHAI_HISTORY_PATH` | `session.history_path` | History file path |
| `SHAI_MAX_HISTORY_TURNS` | `session.max_history_turns` | Max history turns |

### Provider Fallback Variables

| Variable | Used By |
|----------|---------|
| `OPENAI_API_KEY` | OpenAI provider (fallback) |
| `ANTHROPIC_API_KEY` | Anthropic provider (fallback) |
| `OLLAMA_API_BASE_URL` | Ollama provider (fallback) |

## Provider-Specific Notes

### OpenAI

```bash
export OPENAI_API_KEY="sk-..."
export SHAI_PROVIDER="openai"
export SHAI_MODEL="gpt-4o"  # or gpt-4o-mini, gpt-4-turbo
```

### Anthropic

```bash
export ANTHROPIC_API_KEY="sk-..."
export SHAI_PROVIDER="anthropic"
export SHAI_MODEL="claude-3-5-sonnet-20241022"
```

### Ollama (Local)

```bash
# Start Ollama server
ollama serve

# Pull a model
ollama pull llama3.1

# Configure shai
export SHAI_PROVIDER="ollama"
export SHAI_MODEL="llama3.1"
export OLLAMA_API_BASE_URL="http://localhost:11434"  # optional
```

## Viewing Current Config

```bash
# Show merged config
shai config show
```

## Config Validation

Run with debug to see effective config:

```bash
shai --debug "test command"
```

The debug output shows the resolved provider, model, and all settings.
