# ADR-005: Non-Streaming for v1

**Date:** 2026-11-08  
**Status:** Accepted

## Context

rig supports streaming token deltas via `stream_completion`.

## Decision

Use non-streaming completion for v1. Add streaming later if needed.

## Rationale

- Simpler implementation (single `await` per turn)
- Tool calls need complete response anyway (can't stream partial JSON)
- Streaming adds complexity for marginal UX gain in REPL

## Consequences

- User sees full response at once (no token-by-token)
- Can upgrade to streaming in v2 without API changes

## Related

- [ADR-010: RAG v2](010-rag-v2.md)