# ADR-006: Dual LLM for Risk Assessment (Separate Config)

**Date:** 2026-11-08  
**Status:** Accepted

## Context

Risk classification could use the same model as the primary agent (self-assessment) or a separate "judge" model with its own configuration.

## Decision

Use **dual LLM** — the risk assessment runs on a separate model with independent configuration (provider, model, temperature, base_url, api_key).

## Rationale

- Can use smaller/faster/cheaper model for risk (e.g., `gpt-4o-mini` or local Ollama) while primary uses `gpt-4o`/`claude-3.5-sonnet`
- Risk model can run at `temperature=0` (deterministic) while primary uses `temperature=0.2`
- Separate provider possible (e.g., primary=OpenAI, risk=Ollama for privacy)
- Isolation: risk model not influenced by primary's conversation context
- Configurable per-project via TOML

## Consequences

- Two provider configs in TOML: `[provider]` and `[risk_provider]`
- Two model instances created at startup
- Slightly more config complexity
- Two API calls per tool invocation (primary generates command, risk classifies it)
- Can fall back to single-LLM if `risk_provider` not configured

## Related

- [ADR-001: LLM-Only Risk](001-llm-only-risk.md)
- [ADR-002: Manual Agent Loop](002-manual-agent-loop.md)
- [ADR-008: Provider Config](008-provider-config.md)