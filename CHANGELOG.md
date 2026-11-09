# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- Initial project structure and core architecture
- Turn-by-turn agent loop with rig 0.46
- Four-tier risk classifier (LLM-based)
- JSONL conversation history (per-directory)
- Dual LLM support (primary + risk model)
- Configuration via TOML + environment variables

### Changed
- N/A

### Fixed
- N/A

### Security
- N/A

## [0.1.0] - 2026-11-08

### Added
- **Core Agent Loop**: Manual turn-by-turn with `max_turns(1)` for full control
- **Risk Classification**: 4-tier LLM-based classifier (Allowed, ReadSensitive, WriteRecoverable, Catastrophic)
- **Dual LLM Architecture**: Separate primary (command generation) and risk (classification) models
- **Configuration System**: TOML (`~/.config/shai/config.toml`) + env vars with priority merge (Env > TOML > Defaults)
- **Providers**: OpenAI, Anthropic, Ollama support with custom base_url and api_key
- **History**: JSONL format in `.shai/session.jsonl` (50 turns max, per-directory)
- **CLI Modes**: One-shot (`shai "prompt"`) and REPL (`shai`)
- **REPL Commands**: `/help`, `/exit`, `/history`, `/clear`, `/config`, `/risk`, `/verbose`, `/debug`, `/reasoning`
- **Flags**: `-v/--verbose`, `-d/--debug`, `-r/--show-reasoning`

### Tools (15 total)
- **Shell**: `shell` (LLM-classified risk 1-4)
- **File Read**: `read`, `glob`, `grep`, `list_dir` (Level 1 - auto)
- **Web**: `web_search`, `web_fetch` (Level 1 - auto)
- **Git Read**: `git_status`, `git_diff`, `git_log` (Level 1 - auto)
- **Git Write**: `git_commit`, `git_push` (Level 3 - confirm)
- **File Write**: `edit`, `patch` (Level 3 - confirm)
- **Tasks**: `todo_read`, `todo_write` (Level 1 - auto)

### Changed
- N/A

### Fixed
- N/A

### Security
- Shell execution via `sh -c` with user confirmation for sensitive operations
- No hardcoded deny/allow patterns — LLM-only risk classification
- Path validation prevents directory traversal
- No secrets stored in history (redacted by risk classifier)

## Version History

| Version | Date | Notes |
|---------|------|-------|
| 0.1.0 | 2026-11-08 | Initial release |

## Planned Releases

### [0.2.0] - Q1 2027
- Git operations fully implemented (currently stubbed)
- Improved error handling and recovery
- Better REPL UX (completion, syntax highlighting)

### [0.3.0] - Q2 2027
- Streaming output support
- Plugin system architecture
- More sophisticated risk prompts

### [1.0.0] - Q3 2027
- Stable API
- Full documentation
- Production-ready

---

## Release Notes Template

```markdown
## [X.Y.Z] - YYYY-MM-DD

### Added
- New features

### Changed
- Changes in existing functionality

### Deprecated
- Soon-to-be removed features

### Removed
- Removed features

### Fixed
- Bug fixes

### Security
- Security improvements
```