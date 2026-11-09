# ADR-002: Manual Turn-by-Turn Agent Loop

**Date:** 2026-11-08  
**Status:** Accepted

## Context

rig 0.46 provides `AgentBuilder` and `AgentRunner` for multi-turn conversations. We need to intercept tool calls for risk checking and user confirmation.

## Decision

Use **explicit manual loop** with `max_turns(1)` instead of rig's hook system.

```rust
loop {
    let runner = agent.prompt(prompt).history(history).max_turns(1);
    let response = runner.await?;
    
    if let Some(tool_call) = response.tool_calls().first() {
        // Our risk logic here - full control
        let result = handle_tool_call(tool_call).await?;
        history.push(Message::tool_result(tool_call.id, result));
        prompt = String::new();
        continue;
    }
    // Final response
    break;
}
```

## Rationale

- Full control over risk/confirm/exec flow (critical for human-in-the-loop)
- `prompt_user()` is blocking stdin read — awkward in async hook callback
- No dependency on rig's internal hook API (unverified in 0.46)
- Explicit loop = easier debugging, each turn visible
- Natural fit for "stop at tool call, handle, continue" pattern

## Consequences

- ~30 lines of loop boilerplate (acceptable)
- Manual history management (`history.push()`)
- Must rebuild runner each turn with updated history

## Related

- [ADR-006: Dual LLM for Risk](006-dual-llm-risk.md)
- [ADR-001: LLM-Only Risk](001-llm-only-risk.md)