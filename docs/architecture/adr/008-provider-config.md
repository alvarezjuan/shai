# ADR-008: Provider Config via TOML + Environment Variables

**Date:** 2026-11-08  
**Status:** Accepted

## Context

Support OpenAI, Anthropic, Ollama with custom base_url and api_key for both primary and risk models.

## Decision

Config file at `~/.config/shai/config.toml` with env var overrides.

```toml
[provider]
type = "openai"        # "openai" | "anthropic" | "ollama"
model = "gpt-4o"       # provider-specific model id
base_url = ""          # optional override
api_key = ""           # optional (env var takes precedence)
temperature = 0.2
max_tokens = 4096

[risk_provider]
type = "openai"        # optional: defaults to [provider] if omitted
model = "gpt-4o-mini"  # smaller/faster model for risk
base_url = ""          # optional override
api_key = ""           # optional (env var takes precedence)
temperature = 0.0      # deterministic for risk classification
max_tokens = 100       # only needs JSON output
```

Env vars for primary: `SHAI_PROVIDER`, `SHAI_MODEL`, `SHAI_BASE_URL`, `SHAI_API_KEY`, `SHAI_TEMPERATURE`, `SHAI_MAX_TOKENS`, plus provider-specific fallbacks (`OPENAI_API_KEY`, `ANTHROPIC_API_KEY`, `OLLAMA_API_BASE_URL`).

Env vars for risk: `SHAI_RISK_PROVIDER`, `SHAI_RISK_MODEL`, `SHAI_RISK_BASE_URL`, `SHAI_RISK_API_KEY`, `SHAI_RISK_TEMPERATURE`, `SHAI_RISK_MAX_TOKENS`.

## Rationale

- TOML is human-editable, supports comments
- Env vars for secrets (never in config file)
- Priority: env > TOML > defaults
- `risk_provider` optional — falls back to `provider` if omitted
- Works for CI/CD and local dev

## Consequences

- Config loading logic needed (priority merge, fallback logic)
- Per-project config not supported yet (global only)
- Two model instances at startup

## Related

- [ADR-007: rig Umbrella](007-rig-umbrella.md)
- [ADR-006: Dual LLM](006-dual-llm-risk.md)