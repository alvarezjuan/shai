# ADR-007: rig 0.46 Umbrella Crate with Agent Feature

**Date:** 2026-11-08  
**Status:** Accepted

## Context

rig ecosystem has `rig-core`, `rig-agent`, and provider crates.

## Decision

Use `rig` umbrella crate v0.46 with features `["agent", "reqwest", "derive"]`.

```toml
rig = { version = "0.46", features = ["agent", "reqwest", "derive"] }
```

## Rationale

- `agent` feature enables `AgentBuilder`, `AgentRunner`, `AgentHook`
- `reqwest` enables HTTP clients for all providers
- `derive` enables `#[rig_tool]` macro for concise tool definitions
- Single dependency, version-aligned subcrates

## Consequences

- Slightly larger binary (includes all provider clients)
- All rig APIs under `rig::` namespace

## Related

- [ADR-008: Provider Config](008-provider-config.md)