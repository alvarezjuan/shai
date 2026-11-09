# Contributing to shai

Thank you for your interest in contributing to shai! This document outlines the process for contributing to the project.

## Code of Conduct

This project follows the [Contributor Covenant Code of Conduct](CODE_OF_CONDUCT.md). By participating, you are expected to uphold this code.

## How to Contribute

### Reporting Bugs

Before creating a bug report, please check the [issue tracker](https://github.com/yourusername/shai/issues) to see if the problem has already been reported.

When creating a bug report, include:

- **Clear title** describing the issue
- **Steps to reproduce** the problem
- **Expected behavior** vs **actual behavior**
- **Environment**: OS, Rust version, shai version
- **Configuration** (redact API keys)
- **Logs/output** if applicable

### Suggesting Features

Feature requests are welcome! Please:

1. Check existing issues and discussions first
2. Clearly describe the feature and its use case
3. Explain how it fits with shai's philosophy (CLI-first, safety-focused)
4. Consider implementation complexity

### Code Contributions

#### Prerequisites

- Rust 1.75+
- `cargo`, `rustfmt`, `clippy` installed
- Basic familiarity with async Rust and the `rig` framework

#### Development Setup

```bash
# Fork and clone
git clone https://github.com/yourusername/shai
cd shai

# Install pre-commit hooks (optional but recommended)
cargo install cargo-husky
cargo husky install

# Verify build
cargo build
cargo test
```

#### Code Style

- Follow standard Rust conventions (`rustfmt`)
- Pass `clippy` checks: `cargo clippy -- -D warnings`
- Write tests for new functionality
- Update documentation for user-facing changes
- Keep functions small and focused
- Use meaningful names (avoid abbreviations)

#### Commit Messages

Follow [Conventional Commits](https://www.conventionalcommits.org/):

```
feat: add web_search tool
fix: handle empty history in REPL
docs: update README with Ollama config
refactor: simplify risk classifier prompt
test: add integration test for git_commit
chore: update dependencies
```

#### Pull Request Process

1. **Create a feature branch** from `main`
2. **Make focused changes** — one logical change per PR
3. **Run tests**: `cargo test` (all must pass)
4. **Run checks**: `cargo clippy -- -D warnings` and `cargo fmt --check`
5. **Update documentation** if needed
6. **Submit PR** with clear description
7. **Address review feedback** promptly

### Testing

```bash
# Run all tests
cargo test

# Run specific test
cargo test tool::shell::tests::test_execute_simple_command

# Run with output
cargo test -- --nocapture

# Integration tests (require API key)
SHAI_API_KEY=... cargo test integration
```

### Documentation

- Update `README.md` for user-facing changes
- Update `docs/architecture/` for architectural changes
- Add doc comments (`///`) for public APIs
- Keep `CHANGELOG.md` updated (maintainers handle releases)

## Project Structure

```
shai/
├── src/
│   ├── main.rs           # CLI entry
│   ├── config.rs         # Config loading
│   ├── provider.rs       # Model factory
│   ├── agent.rs          # Agent loop
│   ├── history.rs        # JSONL history
│   ├── repl.rs           # REPL
│   ├── risk/             # Risk classification
│   └── tool/             # 15 tools
├── tests/                # Integration tests
└── docs/                 # Architecture docs
```

## Architecture Guidelines

### Core Principles

1. **Safety First** — Risk classification is mandatory for shell commands
2. **Human-in-the-Loop** — No unattended destructive operations
3. **CLI-First** — No GUI/TUI; optimize for terminal UX
4. **Local-First** — History and config stay on user's machine
5. **Provider-Agnostic** — Support multiple LLM providers

### Adding New Tools

1. Create `src/tool/your_tool.rs` following existing patterns
2. Implement `rig::tool::Tool` trait with `NAME`, `description()`, `parameters()`
3. Register in `src/tool/mod.rs` dispatch
4. Add risk level handling in `handle_*` function
5. Write unit tests
6. Update `README.md` tool table

### Risk Levels for New Tools

| Level | When to Use |
|-------|-------------|
| 1 (Allowed) | Read-only, no side effects, no sensitive data |
| 2 (ReadSensitive) | Reads secrets, PII, auth tokens |
| 3 (WriteRecoverable) | Writes/deletes but recoverable |
| 4 (Catastrophic) | Irrecoverable destruction |

## Release Process

Maintainers handle releases:

1. Update `CHANGELOG.md`
2. Bump version in `Cargo.toml`
3. Create git tag: `git tag v0.x.y`
4. Push tag: `git push origin v0.x.y`
5. GitHub Actions publishes to crates.io

## Getting Help

- **Questions**: [GitHub Discussions](https://github.com/yourusername/shai/discussions)
- **Bugs**: [Issue Tracker](https://github.com/yourusername/shai/issues)
- **Security**: See [SECURITY.md](SECURITY.md)

## Recognition

Contributors are recognized in:
- `CHANGELOG.md` for each release
- GitHub's contributor graph
- Special thanks in release notes

Thank you for contributing to shai! 🚀