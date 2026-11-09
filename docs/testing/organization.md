# Test Organization

How tests are organized in the shai codebase.

---

## File Structure

```
src/
├── config.rs              # #[cfg(test)] mod tests { config loading }
├── provider.rs            # #[cfg(test)] mod tests { provider factory }
├── agent.rs               # #[cfg(test)] mod tests { agent loop }
├── history.rs             # #[cfg(test)] mod tests { JSONL load/save }
├── repl.rs                # #[cfg(test)] mod tests { REPL commands }
├── risk/
│   ├── classifier.rs      # #[cfg(test)] mod tests { risk eval }
│   └── prompt.rs          # #[cfg(test)] mod tests { prompt template }
└── tool/
    ├── mod.rs             # #[cfg(test)] mod tests { dispatch }
    ├── shell.rs           # shell execution + risk
    ├── read.rs            # file reading
    ├── glob.rs            # pattern matching
    ├── grep.rs            # content search
    ├── list_dir.rs        # directory listing
    ├── web_search.rs      # web search
    ├── web_fetch.rs       # URL fetching
    ├── git.rs             # git operations
    ├── edit.rs            # file editing
    └── todo.rs            # task tracking

tests/
├── integration.rs         # Full flow tests
└── contract.rs            # (planned) contract tests
```

---

## Module Conventions

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::Builder;
    use std::fs;
    use std::env;

    // Each test function: one behavior, one assertion focus
    #[tokio::test]
    async fn test_<function>_<scenario>_<expected>() { ... }
}
```

### Naming Convention

```
test_<function>_<scenario>_<expected>
```

Examples:
- `test_read_tool_valid_path_returns_content`
- `test_read_tool_invalid_path_returns_error`
- `test_risk_classifier_allowed_command_returns_level_1`
- `test_shell_tool_timeout_returns_timeout_error`

---

## Test Template

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::Builder;
    use std::fs;
    use std::env;

    #[tokio::test]
    async fn test_my_function_valid_input_returns_expected() {
        // Arrange
        let cwd = env::current_dir().unwrap();
        let dir = Builder::new().prefix("shai_test_").tempdir_in(&cwd).unwrap();
        let file = dir.path().join("test.txt");
        fs::write(&file, "test content").unwrap();

        // Act
        let tool = MyTool;
        let mut ctx = ToolContext::default();
        let args = MyArgs { path: file.to_string_lossy().to_string() };
        let result = tool.call(&mut ctx, args).await.unwrap();

        // Assert
        assert_eq!(result.field, "expected");
    }

    #[tokio::test]
    async fn test_my_function_invalid_input_returns_error() {
        let tool = MyTool;
        let mut ctx = ToolContext::default();
        let args = MyArgs { path: "/invalid/path".into() };
        let result = tool.call(&mut ctx, args).await;
        assert!(result.is_err());
    }
}
```

---

## Key Conventions

| Convention | Description |
|------------|-------------|
| **Location** | Unit tests in `#[cfg(test)]` modules alongside source code |
| **Naming** | `test_<function>_<scenario>_<expected>` |
| **Isolation** | Each test creates its own temp directory within CWD |
| **Async** | Use `#[tokio::test]` for async tests |
| **Assertions** | Use `assert_eq!`, `assert!(result.is_ok())`, `assert!(result.is_err())` |
| **Path Validation** | File tools validate paths are within CWD |

---

## Path Validation Requirement

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

---

## Test Targets

| Target | Command | Description |
|--------|---------|-------------|
| Unit tests | `cargo test --lib` | All `#[cfg(test)]` modules |
| Integration | `cargo test --test integration` | `tests/integration.rs` |
| Contract (planned) | `cargo test --test contract` | `tests/contract.rs` |
| All | `cargo test` | All targets |

---

## Running Tests

```bash
# All tests
cargo test

# Unit tests only
cargo test --lib

# Specific test
cargo test tool::shell::tests::test_execute_simple_command

# All tests in a module
cargo test tool::shell::tests

# With output
cargo test -- --nocapture

# With specific thread count
cargo test -- --test-threads=4
```

---

## Debugging Test Failures

```bash
# Run with backtrace
RUST_BACKTRACE=1 cargo test tool::edit::tests::test_edit_tool -- --nocapture

# Run single test with output
cargo test tool::edit::tests::test_edit_tool -- --nocapture

# Run with debug logging
RUST_LOG=debug cargo test tool::shell::tests -- --nocapture
```

---

## References

- [Testing Philosophy](./philosophy.md)
- [Test Levels](./levels.md)
- [Component Strategies](./components.md)