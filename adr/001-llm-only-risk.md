# ADR-001: LLM-Only Risk Classification

**Date:** 2026-11-08  
**Status:** Accepted

## Context

The shell agent needs a four-tier risk classifier (Allowed, ReadSensitive, WriteRecoverable, Catastrophic) to decide whether to auto-execute, prompt for confirmation, or deny shell commands.

## Decision

Use **LLM-only risk classification** with a single structured prompt containing few-shot examples. No hardcoded regex deny/allow lists.

## Rationale

- LLM can reason about context (cwd, recent history, command semantics) better than static patterns
- Few-shot calibration in prompt is easier to maintain than regex lists
- Single LLM call is fast and cheap with modern models
- Avoids false positives/negatives from pattern matching (e.g., `rm -rf /tmp/build` vs `rm -rf /`)

## Consequences

- Requires well-calibrated few-shot examples in prompt
- Slight latency for risk classification call (~100-300ms)
- No hard guardrail — relies entirely on LLM judgment

## Related

- [ADR-006: Dual LLM for Risk](006-dual-llm-risk.md)
- [ADR-002: Manual Agent Loop](002-manual-agent-loop.md)