# Quality Gates

Quality gates and requirements for the shai project.

---

## Pre-commit (Local)

```bash
# Required before every commit
cargo fmt --check && cargo clippy -- -D warnings && cargo test --lib
```

Or use the Makefile:
```bash
make pre-commit
```

---

## PR Requirements

- [ ] All CI checks pass
- [ ] No new warnings (`cargo clippy -- -D warnings`)
- [ ] Code formatted (`cargo fmt --check`)
- [ ] Tests added for new functionality
- [ ] Documentation updated (README, AGENTS.md, CHANGELOG.md)
- [ ] No breaking changes without version bump

---

## CI Gates (GitHub Actions)

| Gate | Command | Required |
|------|---------|----------|
| Format | `cargo fmt --check` | ✅ |
| Lint | `cargo clippy -- -D warnings` | ✅ |
| Unit Tests | `cargo test --lib` | ✅ |
| Build Release | `cargo build --release` | ✅ |

---

## Release Gates

- [ ] All CI checks pass on `main`
- [ ] CHANGELOG.md updated with new version
- [ ] Version bumped in `Cargo.toml` (semver)
- [ ] Git tag created: `git tag vX.Y.Z`
- [ ] GitHub Release created with notes
- [ ] Published to crates.io (maintainer only)

---

## Coverage Gates

| Component | Minimum |
|-----------|---------|
| Core (config, history, risk) | 90% |
| Tools (shell, file, web) | 80% |
| Agent loop | 70% |
| Overall | 80% |

> **Note**: Coverage tracking requires `cargo-tarpaulin` (not yet integrated)

---

## Flaky Test Policy

| Situation | Action |
|-----------|--------|
| Test fails intermittently | Fix immediately or delete |
| Test flaky > 2 runs | Delete and rewrite |
| Quarantine flaky tests | ❌ Never quarantine |

---

## Performance Gates

| Metric | Threshold |
|--------|-----------|
| Unit test suite | <10s |
| Integration tests | <30s |
| Release build | <120s |
| CI total | <3min |

---

## Dependency Gates

| Check | Tool |
|-------|------|
| Vulnerabilities | `cargo audit` (in CI) |
| License compliance | `cargo deny` (planned) |
| Outdated deps | `cargo outdated` (manual) |

---

## Monitoring

### Track in CI
- Test execution time (alert if >60s)
- Flaky test rate (target: 0)
- Coverage regression (alert on drop >5%)

### Local
```bash
# Quick check
cargo test --lib

# Full check
cargo fmt --check && cargo clippy -- -D warnings && cargo test --lib

# Coverage (requires cargo-tarpaulin)
cargo tarpaulin --out Html
```

---

## Exceptions

Emergency hotfixes may bypass gates with:
1. Maintainer approval
2. Follow-up issue created
3. Gates re-enabled within 24h

---

## References

- [Testing Philosophy](./philosophy.md)
- [CI/CD Integration](./ci-cd.md)
- [Testing Strategy](../strategy.md)