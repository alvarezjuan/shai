# Test Levels

Detailed breakdown of each test level in the shai testing strategy.

---

## 1. Unit Tests (Primary)

**Location**: `#[cfg(test)]` modules in source files (`src/**/*.rs`)

**Scope**: Single functions, types, pure logic

**Characteristics**:
- No I/O (or mocked)
- Run in <100ms each
- No external dependencies
- Test within CWD for path-validated tools

**Examples**:
- `risk::classifier::tests::test_extract_json_with_markdown`
- `tool::shell::tests::test_execute_simple_command`
- `tool::read::tests::test_read_tool_with_offset_limit`

**Coverage targets**:
- All public functions
- All error paths
- All edge cases (empty, max, invalid)
- All risk level transitions

---

## 2. Integration Tests (Secondary)

**Location**: `tests/integration.rs` (separate test target)

**Scope**: Multi-component flows, real I/O

**Characteristics**:
- Real shell execution
- Real file system operations
- Real HTTP requests (web tools)
- May require API keys (optional in CI)

**Examples**:
- One-shot command flow
- REPL session simulation
- History persistence/load
- Config loading priority

**Run**: `cargo test --test integration` (separate target)

---

## 3. Contract Tests (Planned)

**Location**: `tests/contract.rs` (planned)

**Scope**: API contracts, serialization, CLI interface

**Examples**:
- JSONL history format compatibility
- Tool schema stability
- CLI argument parsing
- Config TOML round-trip

---

## 4. E2E / Manual (Minimal)

**Scope**: Full user workflows

**Run**: Manual or scheduled

**Examples**:
- Full REPL session
- Complex multi-turn tasks
- Provider switching
- Config file scenarios

---

## Comparison

| Aspect | Unit | Integration | Contract | E2E |
|--------|------|-------------|----------|-----|
| **Speed** | <100ms | 1-30s | <1s | 30s-5min |
| **Dependencies** | None | Real I/O | Schema defs | Full stack |
| **Isolation** | Full | Partial | Full | None |
| **Determinism** | High | Medium | High | Low |
| **Maintenance** | Low | Medium | Low | High |
| **Count Target** | 80% | 15% | 5% | 5% |

---

## Running Each Level

```bash
# Unit tests (default)
cargo test --lib

# Integration tests
cargo test --test integration

# Contract tests (planned)
cargo test --test contract

# All tests
cargo test

# Specific test
cargo test tool::shell::tests::test_execute_simple_command
```

---

## References

- [Testing Philosophy](./philosophy.md)
- [Component Strategies](./components.md)
- [CI/CD Integration](./ci-cd.md)