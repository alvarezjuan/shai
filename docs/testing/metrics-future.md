# Metrics & Future Improvements

Tracking metrics and planned improvements for the testing infrastructure.

---

## Current Metrics

### Coverage Goals

| Component | Target | Current |
|-----------|--------|---------|
| Core (config, history, risk) | 90% | TBD |
| Tools (shell, file, web) | 80% | TBD |
| Agent loop | 70% | TBD |
| Overall | 80% | TBD |

> **Note**: Coverage measurement requires `cargo-tarpaulin` (not yet integrated)

### Execution Time Targets

| Test Suite | Target | Alert Threshold |
|------------|--------|-----------------|
| Unit tests | <10s | >60s |
| Integration tests | <30s | >60s |
| Release build | <120s | >180s |
| Full CI pipeline | <3min | >5min |

### Reliability Targets

| Metric | Target |
|--------|--------|
| Flaky test rate | 0 |
| Test pass rate | 100% on main |
| Coverage regression | Alert on drop >5% |

---

## Monitoring

### CI Monitoring

Track in GitHub Actions:
- Test execution time trends
- Flaky test detection
- Coverage reports (when implemented)

### Local Development

```bash
# Quick check
cargo test --lib

# Full check (pre-commit)
cargo fmt --check && cargo clippy -- -D warnings && cargo test --lib

# Coverage (requires cargo-tarpaulin)
cargo tarpaulin --out Html --output-dir coverage/
```

---

## Future Improvements

### Short Term (Q1 2027)

- [ ] Add `cargo-tarpaulin` for coverage reports in CI
- [ ] Add `cargo-mutants` for mutation testing
- [ ] Contract tests for CLI/JSONL format
- [ ] Property-based testing for parsers (`proptest`)

### Medium Term (Q2-Q3 2027)

- [ ] Mutation testing in CI pipeline
- [ ] Visual regression testing for REPL output
- [ ] Chaos testing for agent loop (fault injection)
- [ ] Performance benchmarks with `cargo bench`

### Long Term (2027+)

- [ ] Fuzzing for input parsers (`cargo-fuzz`)
- [ ] Contract testing with LLM providers
- [ ] Automated risk prompt calibration
- [ ] Automated test generation from specs

---

## Tooling Wishlist

| Tool | Purpose | Status |
|------|---------|--------|
| `cargo-tarpaulin` | Code coverage | Planned |
| `cargo-mutants` | Mutation testing | Planned |
| `cargo-fuzz` | Fuzzing | Future |
| `cargo-bench` | Benchmarks | Future |
| `proptest` | Property-based testing | Planned |
| `rstest` | Parametrized tests | Available |
| `mockito`/`wiremock` | HTTP mocking | Planned |
| `cargo-audit` | Vulnerability scanning | In CI |
| `cargo-deny` | License/dependency policy | Planned |

---

## Test Infrastructure Investments

### Priority 1: Coverage
```bash
cargo install cargo-tarpaulin
cargo tarpaulin --out Html --output-dir coverage/
```

### Priority 2: Mutation Testing
```bash
cargo install cargo-mutants
cargo mutants --json
```

### Priority 3: Property-Based Testing
```toml
# Cargo.toml
[dev-dependencies]
proptest = "1.0"
rstest = "0.18"
```

```rust
use proptest::prelude::*;
use rstest::rstest;

#[rstest]
#[case(RiskLevel::Allowed, "ls")]
#[case(RiskLevel::ReadSensitive, "cat /etc/passwd")]
fn test_risk_classification(#[case] expected: RiskLevel, #[case] command: &str) {
    let result = classify(command).await.unwrap();
    assert_eq!(result.level, expected);
}
```

---

## Budget & Resources

| Investment | Estimate | ROI |
|------------|----------|-----|
| Coverage tooling | 1 day | High - confidence in refactors |
| Mutation testing | 2 days | High - catches subtle bugs |
| Property testing | 3 days | Medium - edge cases |
| Fuzzing | 5 days | Medium - security |
| Benchmarks | 2 days | Medium - perf regressions |

---

## Decision Log

| Date | Decision | Rationale |
|------|----------|-----------|
| 2026-11-08 | 80/15/5 pyramid | Focus on fast unit tests |
| 2026-11-08 | In-file unit tests | Co-location with code |
| 2026-11-08 | CWD temp dirs | Path validation requirement |
| 2026-11-08 | No CI external APIs | Deterministic, fast CI |

---

## References

- [Testing Philosophy](./philosophy.md)
- [Component Strategies](./components.md)
- [CI/CD Integration](./ci-cd.md)
- [Quality Gates](./quality-gates.md)