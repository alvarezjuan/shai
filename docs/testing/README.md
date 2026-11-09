# Testing Documentation

This folder contains testing guides and documentation for the shai project.

## Contents

| File | Description |
|------|-------------|
| [philosophy.md](philosophy.md) | Core principles, testing pyramid, decision criteria |
| [levels.md](levels.md) | Test levels: Unit, Integration, Contract, E2E |
| [organization.md](organization.md) | File structure, conventions, templates, running tests |
| [components.md](components.md) | Per-component testing strategies |
| [ci-cd.md](ci-cd.md) | CI/CD integration, GitHub Actions, quality gates |
| [quality-gates.md](quality-gates.md) | Pre-commit, PR, release, coverage gates |
| [metrics-future.md](metrics-future.md) | Metrics tracking, future improvements, tooling |
| [templates.md](templates.md) | Reusable test patterns and templates |

---

## Quick Reference

### Running Tests

```bash
# All tests
cargo test

# Unit tests only
cargo test --lib

# Specific test
cargo test tool::shell::tests::test_execute_simple_command

# With output
cargo test -- --nocapture

# Run specific test module
cargo test tool::shell::tests

# Run with specific thread count
cargo test -- --test-threads=4
```

### Test Organization

```
src/
├── tool/
│   ├── shell.rs          # #[cfg(test)] mod tests { ... }
│   ├── read.rs           # #[cfg(test)] mod tests { ... }
│   ├── glob.rs           # #[cfg(test)] mod tests { ... }
│   ├── grep.rs           # #[cfg(test)] mod tests { ... }
│   ├── list_dir.rs       # #[cfg(test)] mod tests { ... }
│   ├── web_search.rs     # #[cfg(test)] mod tests { ... }
│   ├── web_fetch.rs      # #[cfg(test)] mod tests { ... }
│   ├── git.rs            # #[cfg(test)] mod tests { ... }
│   ├── edit.rs           # #[cfg(test)] mod tests { ... }
│   └── todo.rs           # #[cfg(test)] mod tests { ... }
├── risk/
│   ├── classifier.rs     # #[cfg(test)] mod tests { ... }
│   └── prompt.rs         # #[cfg(test)] mod tests { ... }
├── history.rs            # #[cfg(test)] mod tests { ... }
├── config.rs             # #[cfg(test)] mod tests { ... } (planned)
└── agent.rs              # #[cfg(test)] mod tests { ... } (planned)

tests/
└── integration.rs        # Integration tests (requires API keys)
```

### Test Conventions

| Convention | Description |
|------------|-------------|
| Location | Unit tests in `#[cfg(test)]` modules alongside source code |
| Naming | `test_<function>_<scenario>_<expected>` |
| Isolation | Each test creates its own temp directory within CWD |
| Async | Use `#[tokio::test]` for async tests |
| Assertions | Use `assert_eq!`, `assert!(result.is_ok())`, `assert!(result.is_err())` |

### Path Validation Requirement

**Critical**: File tools validate paths are within the current working directory. All filesystem tests **must** create temporary directories within the current working directory:

```rust
use tempfile::Builder;
use std::env;

let cwd = env::current_dir().unwrap();
let dir = Builder::new().prefix("shai_test_").tempdir_in(&cwd).unwrap();
let file = dir.path().join("test.txt");
fs::write(&file, "content").unwrap();
// ... test using file.to_string_lossy().to_string()
```

This is required because `read_tool`, `edit_tool`, etc. reject paths outside CWD.

### Running Specific Tests

```bash
# Run all tests
cargo test

# Run only unit tests
cargo test --lib

# Run specific test
cargo test tool::shell::tests::test_execute_simple_command

# Run all tests in a module
cargo test tool::shell::tests

# Run with output
cargo test -- --nocapture

# Run with specific thread count
cargo test -- --test-threads=1
```

### Writing New Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::Builder;
    use std::fs;
    use std::env;

    #[tokio::test]
    async fn test_your_tool_basic() {
        // Setup: create temp dir within CWD
        let cwd = env::current_dir().unwrap();
        let dir = Builder::new().prefix("shai_test_").tempdir_in(&cwd).unwrap();
        let file = dir.path().join("test.txt");
        fs::write(&file, "test content").unwrap();

        // Execute
        let tool = YourTool;
        let mut ctx = ToolContext::default();
        let args = YourArgs { path: file.to_string_lossy().to_string() };
        let result = tool.call(&mut ctx, args).await.unwrap();

        // Assert
        assert_eq!(result.field, "expected");
    }

    #[tokio::test]
    async fn test_your_tool_error_case() {
        let tool = YourTool;
        let mut ctx = ToolContext::default();
        let args = YourArgs { path: "/invalid/path".into() };
        let result = tool.call(&mut ctx, args).await;
        assert!(result.is_err());
    }
}
```

### CI Testing

```yaml
# .github/workflows/test.yml (example)
- name: Run tests
  run: |
    cargo test --lib
    cargo test -- --test-threads=4
```

### Debugging Test Failures

```bash
# Run with backtrace
RUST_BACKTRACE=1 cargo test tool::edit::tests::test_edit_tool -- --nocapture

# Run single test with output
cargo test tool::edit::tests::test_edit_tool -- --nocapture

# Run with debug logging
RUST_LOG=debug cargo test tool::shell::tests -- --nocapture
```

### Test Coverage

```bash
# Install cargo-tarpaulin
cargo install cargo-tarpaulin

# Run coverage
cargo tarpaulin --out Html --output-dir coverage/
```

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

## Key Files

- **Philosophy**: [philosophy.md](philosophy.md) — Core principles, pyramid, decision criteria
- **Levels**: [levels.md](levels.md) — Unit, Integration, Contract, E2E
- **Organization**: [organization.md](organization.md) — File structure, conventions, running tests
- **Components**: [components.md](components.md) — Per-component strategies
- **CI/CD**: [ci-cd.md](ci-cd.md) — GitHub Actions, quality gates
- **Quality Gates**: [quality-gates.md](quality-gates.md) — Pre-commit, PR, release requirements
- **Metrics/Future**: [metrics-future.md](metrics-future.md) — Metrics, tooling, roadmap
- **Templates**: [templates.md](templates.md) — Reusable test patterns

---

## Related Documentation

| Document | Path | Description |
|----------|------|-------------|
| **Superpowers Documentation** | [../../superpowers/README.md](../../superpowers/README.md) | Superpowers methodology: specs, plans, workflow |
| **Architecture Documentation** | [../architecture/README.md](../architecture/README.md) | Architecture docs, ADRs, diagrams |
| **Main Project README** | [../../README.md](../../README.md) | Project overview, quick start, usage |
| **Testing README (this file)** | [README.md](README.md) | Self-reference for internal links |
| **Main AGENTS.md** | [../../AGENTS.md](../../AGENTS.md) | Agent development guide |