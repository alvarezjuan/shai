# CI/CD Integration

How testing integrates with the CI/CD pipeline.

---

## GitHub Actions Workflow

```yaml
# .github/workflows/ci.yml
name: CI

on: [push, pull_request]

jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
      - name: Cache
        uses: Swatinem/rust-cache@v2
      - name: Check formatting
        run: cargo fmt --check
      - name: Run clippy
        run: cargo clippy -- -D warnings
      - name: Run tests
        run: cargo test --lib
      - name: Build release
        run: cargo build --release
```

---

## Test Stages

| Stage | Command | Timeout |
|-------|---------|---------|
| Format | `cargo fmt --check` | 30s |
| Lint | `cargo clippy -- -D warnings` | 60s |
| Unit Tests | `cargo test --lib` | 60s |
| Build | `cargo build --release` | 120s |

---

## Local Pre-commit

```bash
# Run before committing
cargo fmt --check && cargo clippy -- -D warnings && cargo test --lib

# Or use the Makefile
make pre-commit
```

---

## PR Requirements Checklist

- [ ] All CI checks pass
- [ ] No new warnings
- [ ] Tests added for new functionality
- [ ] Documentation updated
- [ ] No breaking changes without version bump

---

## Release Checklist

- [ ] All tests pass
- [ ] CHANGELOG.md updated
- [ ] Version bumped in Cargo.toml
- [ ] Git tag created
- [ ] GitHub release created

---

## Test Execution Time

| Test Suite | Expected Time |
|------------|---------------|
| Unit tests | <10s |
| Integration | <30s |
| Build (release) | <120s |
| Full CI | <3min |

### Alerts
- Alert if test execution >60s
- Alert if build >120s
- Fail on any flaky test

---

## Caching

```yaml
- name: Cache
  uses: Swatinem/rust-cache@v2
  with:
    key: ${{ runner.os }}-cargo-${{ hashFiles('**/Cargo.lock') }}
```

---

## Dependencies

```yaml
- uses: dtolnay/rust-toolchain@stable
  with:
    components: rustfmt, clippy
```

---

## Artifacts

```yaml
- name: Upload coverage
  uses: actions/upload-artifact@v4
  with:
    name: coverage
    path: coverage/
```

---

## References

- [Testing Philosophy](./philosophy.md)
- [Test Levels](./levels.md)
- [Quality Gates](./quality-gates.md)