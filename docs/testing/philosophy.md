# Testing Philosophy

Core principles and the testing pyramid for the shai project.

---

## Core Principles

1. **Test at the right level** — Unit tests for logic, integration tests for flows, manual for UX
2. **Fast feedback** — Unit tests run in <10s; CI completes in <2min
3. **Deterministic** — Tests must pass/fail consistently; flaky tests are bugs
4. **Isolated** — No shared state between tests; each test owns its setup
5. **Meaningful names** — `test_<what>_<condition>_<expected>` tells the story

---

## Testing Pyramid

```
         ████████
        ██████████      E2E / Manual (few)
       ████████████
      ██████████████    Integration (some)
     ████████████████
    ██████████████████  Unit (many) ← Focus here
   ████████████████████
  ██████████████████████
 ████████████████████████
```

**Target distribution**: 80% unit / 15% integration / 5% E2E

---

## Decision Criteria

| Test Type | When to Use |
|-----------|-------------|
| **Unit** | Pure logic, pure functions, error handling, serialization, parsing |
| **Integration** | Multi-component flows, real I/O, shell execution, HTTP requests |
| **Contract** | API stability, schema compatibility, CLI interface, serialization formats |
| **E2E/Manual** | Full user workflows, UX validation, provider switching, complex multi-turn |

---

## Guiding Questions

When deciding where a test belongs, ask:

1. **Does it need real I/O?** → Integration
2. **Does it test a contract/schema?** → Contract
3. **Does it test a full user workflow?** → E2E/Manual
4. **Otherwise** → Unit

---

## References

- [Test Levels](./levels.md)
- [Component Strategies](./components.md)
- [CI/CD Integration](./ci-cd.md)