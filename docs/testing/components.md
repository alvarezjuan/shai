# Component Testing Strategies

Detailed testing strategies for each component in the shai codebase.

---

## Risk Classifier (`src/risk/`)

### Unit Tests
- JSON parsing: valid, markdown-wrapped, extra text
- Risk level enum serialization
- Prompt template rendering
- Few-shot example stability

### Integration
- Real LLM calls (manual/optional)
- Prompt calibration verification

**Strategy**: Test JSON extraction exhaustively; LLM behavior tested manually

---

## Tool Registry (`src/tool/mod.rs`)

### Unit Tests
- Dispatch routing for all 15 tools
- Risk level handling per tool
- Confirmation prompt logic
- Error propagation

**Mock Strategy**: Use real tool implementations with controlled inputs

---

## Shell Tool (`src/tool/shell.rs`)

### Unit Tests
- Command execution: success, failure, stderr, timeout
- Output capture: stdout, stderr, exit code
- Timeout handling

### Integration
- Real shell commands
- Pipe/redirect/builtin support
- Risk classifier integration

**Safety**: Tests run harmless commands (`echo`, `false`, `sleep`)

---

## File Tools (`read`, `glob`, `grep`, `list_dir`)

### Unit Tests
- Path validation (within CWD, traversal rejection)
- Content reading: offset, limit, encoding
- Pattern matching: glob syntax, regex
- Metadata: size, modified time, sorting

**Critical**: All temp dirs created within CWD using `Builder::tempdir_in(&cwd)`

---

## Web Tools (`web_search`, `web_fetch`)

### Unit Tests
- Argument parsing
- Response parsing (HTML → text)
- Error handling: timeout, HTTP errors, invalid URLs

### Integration (manual)
- Real HTTP requests
- Rate limiting behavior
- Content extraction quality

**Mock**: Consider `mockito` or `wiremock` for CI

---

## Git Tools (`git.rs`)

### Current
Stubs returning placeholder messages

### Future Tests
- Repository initialization
- Status/diff/log output parsing
- Commit/push with confirmation
- Error handling: not a repo, conflicts

---

## Edit/Patch Tools

### Unit Tests
- String replacement: single, multiple, not found
- Diff generation: unified format
- Patch application: valid, malformed, conflicts
- Path validation

---

## Config System (`src/config.rs`)

### Unit Tests
- Default values
- TOML loading: global, project
- Env var override priority
- Risk provider fallback
- Type parsing: bool, int, float

---

## History (`src/history.rs`)

### Unit Tests
- Load: empty, missing, valid JSONL
- Save: create dirs, append format
- Turn limiting: 50 turns = 200 messages
- Message serialization round-trip

---

## Agent Loop (`src/agent.rs`)

### Planned Unit Tests
- Tool call detection
- Risk integration
- History management
- Turn termination

### Integration
- Mock model returning controlled responses
- Multi-turn conversation flow

---

## REPL (`src/repl.rs`)

### Planned Unit Tests
- Command parsing: `/exit`, `/help`, `/history`, `/risk`
- Flag toggling
- History file management

---

## Coverage Goals

| Component | Target |
|-----------|--------|
| Core (config, history, risk) | 90% |
| Tools (shell, file, web) | 80% |
| Agent loop | 70% |
| Overall | 80% |

---

## References

- [Testing Philosophy](./philosophy.md)
- [Test Levels](./levels.md)
- [Test Organization](./organization.md)