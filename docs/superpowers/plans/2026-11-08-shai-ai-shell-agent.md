# shai AI Shell Agent Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Build a stateful CLI agent (`shai`) that translates natural language to shell commands, executes them via `sh -c` with a 4-tier LLM-based risk classifier + user confirmation, and persists JSONL conversation history per-directory.

**Architecture:** Manual turn-by-turn loop using rig 0.46's `AgentBuilder` with `max_turns(1)` for full control over risk/confirm/exec flow. Dual LLM setup: primary model for command generation, separate risk model for classification. 15 v1 tools registered via `#[rig_tool]`. Config via TOML + env vars with priority merge (env > TOML > defaults). History as JSONL in `.shai/session.jsonl`.

**Tech Stack:** Rust 2021, rig 0.46 (features: `agent`, `reqwest`, `derive`), tokio, clap, serde, schemars, thiserror, anyhow, tracing, rustyline, walkdir, ignore, reqwest, scraper, url, git2

**Spec:** `docs/superpowers/specs/2026-11-08-ai-shell-agent-design.md`

---

## Global Constraints

- Rust 2021 edition, rig 0.46 umbrella crate with features `["agent", "reqwest", "derive"]`
- Single binary, statically linked where possible
- History stored as JSONL (one `rig_core::completion::Message` per line)
- Four risk levels: 1=Allowed, 2=ReadSensitive, 3=WriteRecoverable, 4=Catastrophic
- Config priority: Environment variables > `~/.config/shai/config.toml` > Built-in defaults
- `risk_provider` optional — falls back to `provider` if omitted
- `sh -c` for shell execution (supports pipes, redirects, builtins)
- Non-streaming for v1 (full response per turn)
- Per-directory history: `.shai/session.jsonl` relative to cwd
- 15 v1 tools: shell, read, glob, grep, list_dir, web_search, web_fetch, git_status, git_diff, git_log, git_commit, git_push, edit, patch, todo_read, todo_write

---

## Review Focus

| Input / Condition | Expected Behavior | Test Owner |
|-------------------|-------------------|------------|
| Command containing `rm -rf /` or `dd if=/dev/zero of=/dev/sda` | Risk level 4 (Catastrophic), denied without prompt | Task 6 (Risk Classifier) |
| Command reading `/etc/passwd` or AWS secrets | Risk level 2 (ReadSensitive), prompts y/N | Task 6 (Risk Classifier) |
| Command `rm file.txt` or `git reset --hard` | Risk level 3 (WriteRecoverable), prompts y/N | Task 6 (Risk Classifier) |
| Config file missing + no env vars | Uses built-in defaults (OpenAI/gpt-4o) | Task 2 (Config) |
| REPL Ctrl+C | Saves history, exits cleanly | Task 10 (REPL) |
| Malformed LLM tool call | Appends parse error to history, asks LLM to retry | Task 8 (Agent Loop) |
| Command timeout (default 30s) | Kills process, returns timeout error to LLM | Task 5 (Shell Tool) |

---

## File Structure Map

| File | Responsibility |
|------|----------------|
| `Cargo.toml` | Dependencies, features |
| `src/main.rs` | CLI entry, arg parsing, dispatch to one-shot/REPL |
| `src/config.rs` | Config structs, TOML + env loading, priority merge |
| `src/provider.rs` | Provider factory → `DynModel<Completion>` for OpenAI/Anthropic/Ollama |
| `src/risk/classifier.rs` | `RiskClassifier::new(model)`, `evaluate(command)` → `RiskLevel` |
| `src/risk/prompt.rs` | Risk prompt template, few-shot examples, JSON parsing |
| `src/history.rs` | JSONL load/save, `Message` serialization, 50-turn limit |
| `src/tool/mod.rs` | Tool registry, dispatch, risk integration |
| `src/tool/shell.rs` | `#[rig_tool] shell_tool`, `ShellArgs`, `ShellOutput`, `ShellError` |
| `src/tool/read.rs` | `#[rig_tool] read_tool` with path validation |
| `src/tool/glob.rs` | `#[rig_tool] glob_tool` with walkdir + ignore |
| `src/tool/grep.rs` | `#[rig_tool] grep_tool` with regex |
| `src/tool/list_dir.rs` | `#[rig_tool] list_dir_tool` |
| `src/tool/web_search.rs` | `#[rig_tool] web_search_tool` |
| `src/tool/web_fetch.rs` | `#[rig_tool] web_fetch_tool` |
| `src/tool/git.rs` | `#[rig_tool] git_status/diff/log/commit/push` |
| `src/tool/edit.rs` | `#[rig_tool] edit_tool`, `patch_tool` |
| `src/tool/todo.rs` | `#[rig_tool] todo_read/write_tool` |
| `src/agent.rs` | `AgentBuilder` setup, turn-by-turn runner, tool dispatch |
| `src/repl.rs` | rustyline REPL, `/commands`, flag toggles |
| `tests/integration.rs` | Smoke tests |

---

## Tasks

### Task 1: Project Setup & Cargo.toml

**Files:**
- Create: `Cargo.toml`
- Create: `src/main.rs` (stub)
- Create: `src/lib.rs` (re-exports)

**Interfaces:**
- Produces: Compilable workspace with all dependencies

- [ ] **Step 1: Write Cargo.toml with all v1 dependencies**

```toml
[package]
name = "shai"
version = "0.1.0"
edition = "2021"

[dependencies]
rig = { version = "0.46", features = ["agent", "reqwest", "derive"] }
tokio = { version = "1", features = ["full"] }
clap = { version = "4", features = ["derive", "env"] }
serde = { version = "1", features = ["derive"] }
serde_json = "1"
schemars = "1"
thiserror = "1"
anyhow = "1"
tracing = "0.1"
tracing-subscriber = { version = "0.3", features = ["env-filter"] }
dirs = "5"
toml = "0.8"
regex = "1"
rustyline = "12"
walkdir = "2"
ignore = "0.12"
reqwest = { version = "0.12", features = ["json", "gzip", "brotli", "native-tls"] }
scraper = "0.20"
url = "2"
git2 = "0.18"
```

- [ ] **Step 2: Create src/lib.rs with module declarations**

```rust
pub mod config;
pub mod provider;
pub mod risk;
pub mod history;
pub mod tool;
pub mod agent;
pub mod repl;
```

- [ ] **Step 3: Create minimal src/main.rs that compiles**

```rust
use clap::Parser;

#[derive(Parser)]
#[command(name = "shai", version, about = "AI shell agent")]
struct Cli {
    #[arg(short, long)]
    verbose: bool,
    #[arg(short, long)]
    debug: bool,
    #[arg(short, long)]
    show_reasoning: bool,
    #[arg(long)]
    provider: Option<String>,
    #[arg(long)]
    model: Option<String>,
    prompt: Option<String>,
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    println!("shai starting... (stub)");
    Ok(())
}
```

- [ ] **Step 4: Run cargo build to verify**

```bash
cargo build
```
Expected: Compiles successfully

- [ ] **Step 5: Commit**

```bash
git add Cargo.toml src/lib.rs src/main.rs
git commit -m "chore: project setup with Cargo.toml and module structure"
```

---

### Task 2: Config System (TOML + Env Vars)

**Files:**
- Create: `src/config.rs`
- Test: `tests/config_test.rs`

**Interfaces:**
- Consumes: —
- Produces: `Config::load() -> Result<Config>`, structs `Config`, `ProviderConfig`, `RiskProviderConfig`, `SessionConfig`, `DisplayConfig`

- [ ] **Step 1: Write failing test for config loading**

```rust
// tests/config_test.rs
use shai::config::{Config, ProviderConfig};

#[test]
fn test_config_defaults() {
    let config = Config::default();
    assert_eq!(config.provider.r#type, "openai");
    assert_eq!(config.provider.model, "gpt-4o");
    assert_eq!(config.provider.temperature, 0.2);
}

#[test]
fn test_config_env_override(monkeypatch) {
    monkeypatch.setenv("SHAI_PROVIDER", "anthropic");
    monkeypatch.setenv("SHAI_MODEL", "claude-3-5-sonnet");
    let config = Config::load().unwrap();
    assert_eq!(config.provider.r#type, "anthropic");
    assert_eq!(config.provider.model, "claude-3-5-sonnet");
}
```

- [ ] **Step 2: Run test to verify it fails**

```bash
cargo test config_test -- --nocapture
```
Expected: FAIL - module not found

- [ ] **Step 3: Implement Config structs and load() in src/config.rs**

Key types:
```rust
#[derive(Debug, Clone, serde::Deserialize)]
pub struct Config {
    pub provider: ProviderConfig,
    pub risk_provider: Option<RiskProviderConfig>,
    pub session: SessionConfig,
    pub display: DisplayConfig,
    // v2: rag, ssh (optional)
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct ProviderConfig {
    pub r#type: String,
    pub model: String,
    pub base_url: Option<String>,
    pub api_key: Option<String>,
    pub temperature: f32,
    pub max_tokens: u32,
}
// ... similar for RiskProviderConfig, SessionConfig, DisplayConfig

impl Config {
    pub fn load() -> anyhow::Result<Self> {
        // 1. Start with defaults
        // 2. Merge ~/.config/shai/config.toml if exists
        // 3. Merge .shai/config.toml if exists (per-project)
        // 4. Override with env vars (SHAI_*, OPENAI_API_KEY, etc.)
        // 5. If risk_provider is None, clone from provider with risk defaults
    }
}
```

Priority merge logic:
- `dirs::config_dir()` for `~/.config/shai/config.toml`
- `std::env::current_dir()` for `.shai/config.toml`
- Env var mapping per spec section 6.2

- [ ] **Step 4: Run test to verify it passes**

```bash
cargo test config_test -- --nocapture
```
Expected: PASS

- [ ] **Step 5: Commit**

```bash
git add src/config.rs tests/config_test.rs
git commit -m "feat: config system with TOML + env var priority merge"
```

---

### Task 3: Provider Factory

**Files:**
- Create: `src/provider.rs`
- Test: `tests/provider_test.rs`

**Interfaces:**
- Consumes: `config::ProviderConfig`, `config::RiskProviderConfig`
- Produces: `build_model(config: &ProviderConfig) -> anyhow::Result<DynModel<Completion>>`

- [ ] **Step 1: Write failing test**

```rust
// tests/provider_test.rs
use shai::provider::build_model;
use shai::config::ProviderConfig;

#[test]
fn test_build_openai_model() {
    let config = ProviderConfig {
        r#type: "openai".into(),
        model: "gpt-4o-mini".into(),
        base_url: None,
        api_key: Some("test-key".into()),
        temperature: 0.0,
        max_tokens: 100,
    };
    let model = build_model(&config);
    assert!(model.is_ok());
}
```

- [ ] **Step 2: Run test to verify it fails**

```bash
cargo test provider_test -- --nocapture
```
Expected: FAIL

- [ ] **Step 3: Implement build_model in src/provider.rs**

```rust
use rig::driver::DynModel;
use rig::operation::Completion;
use rig::providers::openai::{OpenAIConfig, OpenAI};
use rig::providers::anthropic::{AnthropicConfig, Anthropic};
use rig::providers::ollama::Ollama;

pub fn build_model(config: &ProviderConfig) -> anyhow::Result<DynModel<Completion>> {
    match config.r#type.as_str() {
        "openai" => {
            let mut client_config = OpenAIConfig::new(config.api_key.clone().unwrap_or_default());
            if let Some(url) = &config.base_url {
                client_config = client_config.with_base_url(url);
            }
            let client = client_config.client();
            Ok(client.chat(&config.model).into())
        }
        "anthropic" => {
            let mut client_config = AnthropicConfig::new(config.api_key.clone().unwrap_or_default());
            if let Some(url) = &config.base_url {
                client_config = client_config.with_base_url(url);
            }
            let client = client_config.client();
            Ok(client.completion(&config.model).into())
        }
        "ollama" => {
            let base_url = config.base_url.clone().unwrap_or_else(|| "http://localhost:11434".into());
            let client = Ollama::new(&base_url);
            Ok(client.completion(&config.model).into())
        }
        _ => anyhow::bail!("Unknown provider type: {}", config.r#type),
    }
}
```

- [ ] **Step 4: Run test to verify it passes**

```bash
cargo test provider_test -- --nocapture
```
Expected: PASS

- [ ] **Step 5: Commit**

```bash
git add src/provider.rs tests/provider_test.rs
git commit -m "feat: provider factory for OpenAI, Anthropic, Ollama"
```

---

### Task 4: History Manager (JSONL)

**Files:**
- Create: `src/history.rs`
- Test: `tests/history_test.rs`

**Interfaces:**
- Consumes: `config::SessionConfig`
- Produces: `History::load(path, max_turns) -> Vec<Message>`, `History::save(path, messages) -> Result<()>`, `History::push(&mut self, message)`

- [ ] **Step 1: Write failing test**

```rust
// tests/history_test.rs
use shai::history::History;
use rig::completion::Message;
use tempfile::tempdir;

#[test]
fn test_history_load_save() {
    let dir = tempdir().unwrap();
    let path = dir.path().join("session.jsonl");
    
    let msg1 = Message::user("hello");
    let msg2 = Message::assistant("hi");
    
    History::save(&path, &[msg1.clone(), msg2.clone()]).unwrap();
    let loaded = History::load(&path, 50).unwrap();
    
    assert_eq!(loaded.len(), 2);
}
```

- [ ] **Step 2: Run test to verify it fails**

```bash
cargo test history_test -- --nocapture
```
Expected: FAIL

- [ ] **Step 3: Implement History in src/history.rs**

```rust
use rig::completion::Message;
use serde_json;
use std::fs;
use std::path::Path;

pub struct History;

impl History {
    pub fn load(path: &Path, max_turns: usize) -> anyhow::Result<Vec<Message>> {
        if !path.exists() {
            return Ok(Vec::new());
        }
        let content = fs::read_to_string(path)?;
        let messages: Vec<Message> = content
            .lines()
            .filter_map(|line| serde_json::from_str(line).ok())
            .collect();
        // Keep last max_turns * 4 messages (roughly max_turns turns)
        let keep = max_turns * 4;
        if messages.len() > keep {
            Ok(messages[messages.len() - keep..].to_vec())
        } else {
            Ok(messages)
        }
    }
    
    pub fn save(path: &Path, messages: &[Message]) -> anyhow::Result<()> {
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        let content = messages
            .iter()
            .map(|m| serde_json::to_string(m).unwrap())
            .collect::<Vec<_>>()
            .join("\n");
        fs::write(path, content)?;
        Ok(())
    }
}
```

- [ ] **Step 4: Run test to verify it passes**

```bash
cargo test history_test -- --nocapture
```
Expected: PASS

- [ ] **Step 5: Commit**

```bash
git add src/history.rs tests/history_test.rs
git commit -m "feat: JSONL history load/save with turn limiting"
```

---

### Task 5: Shell Tool & Execution

**Files:**
- Create: `src/tool/shell.rs`
- Test: `tests/tool_shell_test.rs`

**Interfaces:**
- Consumes: —
- Produces: `shell_tool` (rig_tool), `execute_command(cmd: &str) -> Result<ShellOutput>`, `ShellArgs`, `ShellOutput`, `ShellError`

- [ ] **Step 1: Write failing test**

```rust
// tests/tool_shell_test.rs
use shai::tool::shell::{execute_command, ShellOutput};

#[tokio::test]
async fn test_execute_simple_command() {
    let output = execute_command("echo hello").await.unwrap();
    assert_eq!(output.stdout.trim(), "hello");
    assert_eq!(output.exit_code, 0);
}

#[tokio::test]
async fn test_execute_failing_command() {
    let output = execute_command("false").await.unwrap();
    assert_ne!(output.exit_code, 0);
}
```

- [ ] **Step 2: Run test to verify it fails**

```bash
cargo test tool_shell_test -- --nocapture
```
Expected: FAIL

- [ ] **Step 3: Implement shell tool in src/tool/shell.rs**

```rust
use rig::tool::contextual::{Tool, ToolContext};
use rig::tool::IntoToolOutput;
use rig::derive::rig_tool;
use serde::{Deserialize, Serialize};
use schemars::JsonSchema;
use thiserror::Error;
use tokio::process::Command;
use tokio::time::{timeout, Duration};

#[derive(Deserialize, JsonSchema)]
pub struct ShellArgs {
    pub command: String,
    #[serde(default = "default_timeout")]
    pub timeout_secs: u64,
}

fn default_timeout() -> u64 { 30 }

#[derive(Serialize, JsonSchema)]
pub struct ShellOutput {
    pub stdout: String,
    pub stderr: String,
    pub exit_code: i32,
}

impl IntoToolOutput for ShellOutput {}

#[derive(Debug, Error)]
pub enum ShellError {
    #[error("Execution failed: {0}")]
    Exec(#[from] std::io::Error),
    #[error("Timeout after {0}s")]
    Timeout(u64),
    #[error("Denied by risk policy: {0}")]
    Denied(String),
    #[error("User denied: {0}")]
    UserDenied(String),
}

#[rig_tool(name = "shell", description = "Execute a shell command via `sh -c` and return stdout, stderr, exit code.")]
async fn shell_tool(
    _ctx: &mut ToolContext,
    args: ShellArgs,
) -> Result<ShellOutput, ShellError> {
    // STUB: Actual execution with risk check happens in AgentLoop
    Ok(ShellOutput { stdout: String::new(), stderr: String::new(), exit_code: 0 })
}

pub async fn execute_command(command: &str) -> Result<ShellOutput, ShellError> {
    let mut cmd = Command::new("sh");
    cmd.arg("-c").arg(command);
    
    let output = timeout(Duration::from_secs(30), cmd.output()).await
        .map_err(|_| ShellError::Timeout(30))??;
    
    Ok(ShellOutput {
        stdout: String::from_utf8_lossy(&output.stdout).to_string(),
        stderr: String::from_utf8_lossy(&output.stderr).to_string(),
        exit_code: output.status.code().unwrap_or(-1),
    })
}
```

- [ ] **Step 4: Run test to verify it passes**

```bash
cargo test tool_shell_test -- --nocapture
```
Expected: PASS

- [ ] **Step 5: Commit**

```bash
git add src/tool/shell.rs tests/tool_shell_test.rs
git commit -m "feat: shell tool with sh -c execution and timeout"
```

---

### Task 6: Risk Classifier (4-Tier LLM-Based)

**Files:**
- Create: `src/risk/prompt.rs`
- Create: `src/risk/classifier.rs`
- Test: `tests/risk_test.rs`

**Interfaces:**
- Consumes: `DynModel<Completion>` (risk model)
- Produces: `RiskClassifier::new(model)`, `evaluate(command: &str, cwd: &str, history: &[String]) -> RiskResult`

- [ ] **Step 1: Write failing test**

```rust
// tests/risk_test.rs
use shai::risk::{RiskClassifier, RiskLevel};

#[tokio::test]
async fn test_risk_allowed() {
    // Uses mock model or real model with known response
    // For unit test, we'll test the prompt building and JSON parsing
}

#[test]
fn test_risk_prompt_json_parsing() {
    let json = r#"{"level": 1, "reason": "read-only command"}"#;
    let result: RiskResult = serde_json::from_str(json).unwrap();
    assert_eq!(result.level, RiskLevel::Allowed);
}
```

- [ ] **Step 2: Run test to verify it fails**

```bash
cargo test risk_test -- --nocapture
```
Expected: FAIL

- [ ] **Step 3: Implement risk types and prompt in src/risk/prompt.rs**

```rust
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum RiskLevel {
    Allowed = 1,
    ReadSensitive = 2,
    WriteRecoverable = 3,
    Catastrophic = 4,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RiskResult {
    pub level: RiskLevel,
    pub reason: String,
}

pub fn build_risk_prompt(command: &str, cwd: &str, recent_commands: &[String]) -> String {
    let history = if recent_commands.is_empty() {
        "none".to_string()
    } else {
        recent_commands.join("\n")
    };
    
    format!(r#"
You are a shell command risk classifier. Classify the command into exactly one level:

1 = Allowed (read-only, no sensitive data, no state change)
2 = ReadSensitive (reads secrets, keys, PII, auth tokens)
3 = WriteRecoverable (writes/deletes but recoverable: git reset, rm file, docker stop)
4 = Catastrophic (irrecoverable: rm -rf /, disk format, fork bomb)

Command: {command}
Working dir: {cwd}
Recent history: {history}

Output JSON only: {{"level": 1, "reason": "..."}}
"#)
}
```

- [ ] **Step 4: Implement RiskClassifier in src/risk/classifier.rs**

```rust
use rig::driver::DynModel;
use rig::operation::Completion;
use rig::completion::Prompt;
use crate::risk::prompt::{build_risk_prompt, RiskLevel, RiskResult};

pub struct RiskClassifier {
    model: DynModel<Completion>,
}

impl RiskClassifier {
    pub fn new(model: DynModel<Completion>) -> Self {
        Self { model }
    }
    
    pub async fn evaluate(
        &self,
        command: &str,
        cwd: &str,
        recent_commands: &[String],
    ) -> anyhow::Result<RiskResult> {
        let prompt = build_risk_prompt(command, cwd, recent_commands);
        let response = self.model.prompt(&prompt).await?;
        let text = response.text();
        
        // Parse JSON from response (may have markdown code fences)
        let json = extract_json(&text)?;
        let result: RiskResult = serde_json::from_str(&json)?;
        Ok(result)
    }
}

fn extract_json(text: &str) -> anyhow::Result<String> {
    // Handle ```json ... ``` or raw JSON
    if let Some(start) = text.find('{') {
        if let Some(end) = text.rfind('}') {
            return Ok(text[start..=end].to_string());
        }
    }
    anyhow::bail!("No JSON found in response: {}", text)
}
```

- [ ] **Step 5: Run test to verify it passes**

```bash
cargo test risk_test -- --nocapture
```
Expected: PASS

- [ ] **Step 6: Commit**

```bash
git add src/risk/prompt.rs src/risk/classifier.rs tests/risk_test.rs
git commit -m "feat: 4-tier LLM-based risk classifier with few-shot prompt"
```

---

### Task 7: File/Read Tools (read, glob, grep, list_dir)

**Files:**
- Create: `src/tool/read.rs`
- Create: `src/tool/glob.rs`
- Create: `src/tool/grep.rs`
- Create: `src/tool/list_dir.rs`
- Test: `tests/tool_file_test.rs`

**Interfaces:**
- Each produces a `#[rig_tool]` function with appropriate args/output

- [ ] **Step 1: Write failing tests for each tool**

```rust
// tests/tool_file_test.rs
use shai::tool::read::read_tool;
use shai::tool::glob::glob_tool;
use shai::tool::grep::grep_tool;
use shai::tool::list_dir::list_dir_tool;
use rig::tool::contextual::ToolContext;
use tempfile::tempdir;

#[tokio::test]
async fn test_read_tool() {
    let dir = tempdir().unwrap();
    let file = dir.path().join("test.txt");
    std::fs::write(&file, "hello world").unwrap();
    
    let mut ctx = ToolContext::default();
    let args = shai::tool::read::ReadArgs { path: file.to_string_lossy().into() };
    let result = read_tool(&mut ctx, args).await.unwrap();
    assert_eq!(result.content, "hello world");
}
```

- [ ] **Step 2: Run tests to verify they fail**

```bash
cargo test tool_file_test -- --nocapture
```
Expected: FAIL

- [ ] **Step 3: Implement each tool**

**read.rs:**
```rust
#[derive(Deserialize, JsonSchema)]
pub struct ReadArgs {
    pub path: String,
    #[serde(default)]
    pub offset: Option<usize>,
    #[serde(default)]
    pub limit: Option<usize>,
}

#[derive(Serialize, JsonSchema)]
pub struct ReadOutput {
    pub content: String,
}

#[rig_tool(name = "read", description = "Read file contents with optional offset/limit.")]
async fn read_tool(_ctx: &mut ToolContext, args: ReadArgs) -> Result<ReadOutput, ReadError> {
    // Validate path is within cwd (no directory traversal)
    // Read file with offset/limit
}
```

**glob.rs:** Use `walkdir` + `ignore` for `.gitignore` support
**grep.rs:** Use `regex` crate, search file contents
**list_dir.rs:** Simple directory listing with metadata

- [ ] **Step 4: Run tests to verify they pass**

```bash
cargo test tool_file_test -- --nocapture
```
Expected: PASS

- [ ] **Step 5: Commit**

```bash
git add src/tool/read.rs src/tool/glob.rs src/tool/grep.rs src/tool/list_dir.rs tests/tool_file_test.rs
git commit -m "feat: file read tools (read, glob, grep, list_dir)"
```

---

### Task 8: Web Tools (web_search, web_fetch)

**Files:**
- Create: `src/tool/web_search.rs`
- Create: `src/tool/web_fetch.rs`
- Test: `tests/tool_web_test.rs`

**Interfaces:**
- Produces: `#[rig_tool]` functions for web search and fetch

- [ ] **Step 1: Write failing tests**

```rust
// tests/tool_web_test.rs
#[tokio::test]
async fn test_web_fetch_tool() {
    // Test with a known URL
}
```

- [ ] **Step 2: Implement web_search.rs** (using reqwest + search API)

```rust
#[derive(Deserialize, JsonSchema)]
pub struct WebSearchArgs {
    pub query: String,
    #[serde(default = "default_max_results")]
    pub max_results: usize,
}

#[rig_tool(name = "web_search", description = "Search the web and return top results.")]
async fn web_search_tool(_ctx: &mut ToolContext, args: WebSearchArgs) -> Result<WebSearchOutput, WebSearchError> {
    // Use duckduckgo html scrape or search API
}
```

- [ ] **Step 3: Implement web_fetch.rs** (using reqwest + scraper)

```rust
#[rig_tool(name = "web_fetch", description = "Fetch and extract text content from a URL.")]
async fn web_fetch_tool(_ctx: &mut ToolContext, args: WebFetchArgs) -> Result<WebFetchOutput, WebFetchError> {
    // GET request, parse HTML with scraper, extract text
}
```

- [ ] **Step 4: Run tests**

```bash
cargo test tool_web_test -- --nocapture
```
Expected: PASS

- [ ] **Step 5: Commit**

```bash
git add src/tool/web_search.rs src/tool/web_fetch.rs tests/tool_web_test.rs
git commit -m "feat: web tools (web_search, web_fetch)"
```

---

### Task 9: Git Tools (status, diff, log, commit, push)

**Files:**
- Create: `src/tool/git.rs`
- Test: `tests/tool_git_test.rs`

**Interfaces:**
- Produces: 5 `#[rig_tool]` functions using `git2` crate

- [ ] **Step 1: Write failing tests**

```rust
// tests/tool_git_test.rs
#[tokio::test]
async fn test_git_status_tool() {
    // Init temp repo, test status
}
```

- [ ] **Step 2: Implement git.rs**

```rust
use git2::Repository;

#[rig_tool(name = "git_status", description = "Show git status.")]
async fn git_status_tool(_ctx: &mut ToolContext, _args: GitStatusArgs) -> Result<GitStatusOutput, GitError> {
    let repo = Repository::open(".")?;
    // Get status
}

#[rig_tool(name = "git_diff", description = "Show git diff.")]
async fn git_diff_tool(_ctx: &mut ToolContext, args: GitDiffArgs) -> Result<GitDiffOutput, GitError> { ... }

#[rig_tool(name = "git_log", description = "Show git log.")]
async fn git_log_tool(_ctx: &mut ToolContext, args: GitLogArgs) -> Result<GitLogOutput, GitError> { ... }

#[rig_tool(name = "git_commit", description = "Commit staged changes.")]
async fn git_commit_tool(_ctx: &mut ToolContext, args: GitCommitArgs) -> Result<GitCommitOutput, GitError> { ... }

#[rig_tool(name = "git_push", description = "Push to remote.")]
async fn git_push_tool(_ctx: &mut ToolContext, args: GitPushArgs) -> Result<GitPushOutput, GitError> { ... }
```

- [ ] **Step 3: Run tests**

```bash
cargo test tool_git_test -- --nocapture
```
Expected: PASS

- [ ] **Step 4: Commit**

```bash
git add src/tool/git.rs tests/tool_git_test.rs
git commit -m "feat: git tools (status, diff, log, commit, push)"
```

---

### Task 10: Edit/Patch Tools

**Files:**
- Create: `src/tool/edit.rs`
- Test: `tests/tool_edit_test.rs`

**Interfaces:**
- Produces: `edit_tool`, `patch_tool` with diff preview

- [ ] **Step 1: Write failing tests**

- [ ] **Step 2: Implement edit.rs**

```rust
#[rig_tool(name = "edit", description = "Edit a file by replacing a string.")]
async fn edit_tool(_ctx: &mut ToolContext, args: EditArgs) -> Result<EditOutput, EditError> {
    // Read file, find old_string, replace with new_string, write back
    // Return diff preview
}

#[rig_tool(name = "patch", description = "Apply a unified diff patch.")]
async fn patch_tool(_ctx: &mut ToolContext, args: PatchArgs) -> Result<PatchOutput, EditError> {
    // Parse unified diff, apply to file
}
```

- [ ] **Step 3: Run tests**

```bash
cargo test tool_edit_test -- --nocapture
```
Expected: PASS

- [ ] **Step 4: Commit**

```bash
git add src/tool/edit.rs tests/tool_edit_test.rs
git commit -m "feat: edit and patch tools with diff preview"
```

---

### Task 11: Todo Tools (todo_read, todo_write)

**Files:**
- Create: `src/tool/todo.rs`
- Test: `tests/tool_todo_test.rs`

**Interfaces:**
- Produces: in-memory task list tools

- [ ] **Step 1: Write failing tests**

- [ ] **Step 2: Implement todo.rs**

```rust
use std::sync::Arc;
use tokio::sync::Mutex;

lazy_static::lazy_static! {
    static ref TODO_LIST: Arc<Mutex<Vec<TodoItem>>> = Arc::new(Mutex::new(Vec::new()));
}

#[rig_tool(name = "todo_read", description = "Read the current todo list.")]
async fn todo_read_tool(_ctx: &mut ToolContext, _args: TodoReadArgs) -> Result<TodoReadOutput, TodoError> { ... }

#[rig_tool(name = "todo_write", description = "Write/replace the todo list.")]
async fn todo_write_tool(_ctx: &mut ToolContext, args: TodoWriteArgs) -> Result<TodoWriteOutput, TodoError> { ... }
```

- [ ] **Step 3: Run tests**

```bash
cargo test tool_todo_test -- --nocapture
```
Expected: PASS

- [ ] **Step 4: Commit**

```bash
git add src/tool/todo.rs tests/tool_todo_test.rs
git commit -m "feat: todo tools (todo_read, todo_write)"
```

---

### Task 12: Tool Registry & Dispatch

**Files:**
- Create: `src/tool/mod.rs`

**Interfaces:**
- Consumes: All tool modules
- Produces: `ToolRegistry`, `dispatch_tool_call(name, args, risk_classifier, config) -> ToolResult`

- [ ] **Step 1: Implement tool/mod.rs**

```rust
pub mod shell;
pub mod read;
pub mod glob;
pub mod grep;
pub mod list_dir;
pub mod web_search;
pub mod web_fetch;
pub mod git;
pub mod edit;
pub mod todo;

use rig::completion::ToolCall;
use crate::risk::RiskClassifier;

pub struct ToolRegistry;

impl ToolRegistry {
    pub async fn dispatch(
        tool_call: &ToolCall,
        risk_classifier: &RiskClassifier,
        config: &Config,
    ) -> anyhow::Result<rig::completion::ToolResult> {
        match tool_call.name.as_str() {
            "shell" => Self::handle_shell(tool_call, risk_classifier, config).await,
            "read" => Self::handle_read(tool_call).await,
            "glob" => Self::handle_glob(tool_call).await,
            "grep" => Self::handle_grep(tool_call).await,
            "list_dir" => Self::handle_list_dir(tool_call).await,
            "web_search" => Self::handle_web_search(tool_call).await,
            "web_fetch" => Self::handle_web_fetch(tool_call).await,
            "git_status" => Self::handle_git_status(tool_call).await,
            "git_diff" => Self::handle_git_diff(tool_call).await,
            "git_log" => Self::handle_git_log(tool_call).await,
            "git_commit" => Self::handle_git_commit(tool_call).await,
            "git_push" => Self::handle_git_push(tool_call).await,
            "edit" => Self::handle_edit(tool_call).await,
            "patch" => Self::handle_patch(tool_call).await,
            "todo_read" => Self::handle_todo_read(tool_call).await,
            "todo_write" => Self::handle_todo_write(tool_call).await,
            _ => anyhow::bail!("Unknown tool: {}", tool_call.name),
        }
    }
    
    async fn handle_shell(...) -> Result { ... } // Risk check, confirm, execute
    async fn handle_read(...) -> Result { ... }  // Direct execute
    // ... etc
}
```

- [ ] **Step 2: Write test for dispatch**

```bash
cargo test tool_dispatch_test -- --nocapture
```

- [ ] **Step 3: Commit**

```bash
git add src/tool/mod.rs tests/tool_dispatch_test.rs
git commit -m "feat: tool registry with risk-integrated dispatch"
```

---

### Task 13: Agent Loop (Turn-by-Turn Runner)

**Files:**
- Create: `src/agent.rs`
- Test: `tests/agent_test.rs`

**Interfaces:**
- Consumes: `Config`, `DynModel<Completion>` (primary), `RiskClassifier`, `ToolRegistry`, `History`
- Produces: `run_agent(prompt: &str, config: &Config) -> Result<String>`

- [ ] **Step 1: Write failing test**

```rust
// tests/agent_test.rs
#[tokio::test]
async fn test_agent_simple_query() {
    // Mock model that returns text response
    // Verify agent returns response
}
```

- [ ] **Step 2: Implement agent.rs**

```rust
use rig::agent::AgentBuilder;
use rig::driver::DynModel;
use rig::operation::Completion;
use rig::completion::{Message, ToolCall, ToolResult};
use crate::config::Config;
use crate::risk::RiskClassifier;
use crate::tool::ToolRegistry;
use crate::history::History;
use std::path::PathBuf;

pub async fn run_agent(prompt: &str, config: &Config) -> anyhow::Result<String> {
    // Build primary model
    let primary_model = crate::provider::build_model(&config.provider)?;
    
    // Build risk model (fallback to primary if not configured)
    let risk_model = if let Some(risk_config) = &config.risk_provider {
        crate::provider::build_model(risk_config)?
    } else {
        primary_model.clone()
    };
    
    let risk_classifier = RiskClassifier::new(risk_model);
    
    // Build agent with primary model and all tools
    let agent = AgentBuilder::new(primary_model)
        .name("shai")
        .preamble(PREAMBLE)
        .temperature(config.provider.temperature)
        .max_tokens(config.provider.max_tokens)
        .tool(crate::tool::shell::shell_tool)
        .tool(crate::tool::read::read_tool)
        .tool(crate::tool::glob::glob_tool)
        .tool(crate::tool::grep::grep_tool)
        .tool(crate::tool::list_dir::list_dir_tool)
        .tool(crate::tool::web_search::web_search_tool)
        .tool(crate::tool::web_fetch::web_fetch_tool)
        .tool(crate::tool::git::git_status_tool)
        .tool(crate::tool::git::git_diff_tool)
        .tool(crate::tool::git::git_log_tool)
        .tool(crate::tool::git::git_commit_tool)
        .tool(crate::tool::git::git_push_tool)
        .tool(crate::tool::edit::edit_tool)
        .tool(crate::tool::edit::patch_tool)
        .tool(crate::tool::todo::todo_read_tool)
        .tool(crate::tool::todo::todo_write_tool)
        .build();
    
    // Load history
    let history_path = PathBuf::from(&config.session.history_path);
    let mut history = History::load(&history_path, config.session.max_history_turns)?;
    
    let mut user_prompt = prompt.to_string();
    
    loop {
        // Run exactly ONE turn
        let runner = agent
            .prompt(user_prompt.clone())
            .history(history.clone())
            .max_turns(1);
        
        let response = runner.await?;
        
        // Check for tool calls
        if let Some(tool_call) = response.tool_calls().first() {
            let tool_result = ToolRegistry::dispatch(tool_call, &risk_classifier, config).await?;
            
            // Feed result back
            history.push(Message::tool_result(tool_call.id.clone(), tool_result));
            user_prompt = String::new();
            continue;
        }
        
        // Final response
        let final_text = response.text();
        println!("{}", final_text);
        
        // Save history
        history.extend(response.messages());
        History::save(&history_path, &history)?;
        
        return Ok(final_text);
    }
}

const PREAMBLE: &str = r#"
You are shai, an AI assistant that executes shell commands to accomplish tasks.
Use the `shell` tool to run commands. The system will evaluate each command for risk
and may ask the user for confirmation before execution.

**Before running commands:**
- Explain what you're doing and why
- If you modify a user's command, explain the change and reason

**After commands execute:**
- Summarize the result (stdout, stderr, exit code)
- Explain what the output means for the task
- Note any errors or unexpected behavior
- Decide next steps based on results

Always be clear and concise in your explanations.
"#;
```

- [ ] **Step 3: Run test**

```bash
cargo test agent_test -- --nocapture
```
Expected: PASS

- [ ] **Step 4: Commit**

```bash
git add src/agent.rs tests/agent_test.rs
git commit -m "feat: turn-by-turn agent loop with risk integration"
```

---

### Task 14: REPL (rustyline)

**Files:**
- Create: `src/repl.rs`
- Modify: `src/main.rs` (wire REPL)

**Interfaces:**
- Consumes: `Config`, `run_agent`
- Produces: `run_repl(config: &Config) -> Result<()>`

- [ ] **Step 1: Write failing test**

```rust
// tests/repl_test.rs
#[test]
fn test_repl_commands() {
    // Test /exit, /help, /config, /risk, /verbose, etc.
}
```

- [ ] **Step 2: Implement repl.rs**

```rust
use rustyline::DefaultEditor;
use rustyline::history::FileHistory;
use crate::config::Config;
use crate::agent::run_agent;

pub async fn run_repl(config: &Config) -> anyhow::Result<()> {
    let mut rl = DefaultEditor::new()?;
    let history_path = dirs::data_dir().unwrap().join("shai").join("repl_history.txt");
    let _ = rl.load_history(&history_path);
    
    println!("shai REPL. Type /help for commands, /exit to quit.");
    
    loop {
        let readline = rl.readline("shai> ");
        match readline {
            Ok(line) => {
                let line = line.trim();
                if line.is_empty() { continue; }
                
                if line.starts_with('/') {
                    if handle_repl_command(line, config, &mut rl, &history_path).await? {
                        break; // exit
                    }
                } else {
                    let _ = run_agent(line, config).await;
                }
                rl.add_history_entry(line)?;
            }
            Err(rustyline::error::ReadlineError::Interrupted) => {
                println!("\nUse /exit to quit");
            }
            Err(rustyline::error::ReadlineError::Eof) => {
                break;
            }
            Err(err) => {
                eprintln!("Error: {}", err);
                break;
            }
        }
    }
    
    rl.save_history(&history_path)?;
    Ok(())
}

async fn handle_repl_command(
    cmd: &str,
    config: &Config,
    rl: &mut DefaultEditor,
    history_path: &std::path::Path,
) -> anyhow::Result<bool> {
    match cmd {
        "/exit" | "/quit" => return Ok(true),
        "/help" => print_help(),
        "/history" => show_history(config)?,
        "/clear" => clear_history(config)?,
        "/config" => println!("{:#?}", config),
        "/risk" => { /* dry-run risk */ },
        "/verbose" => { /* toggle */ },
        "/debug" => { /* toggle */ },
        "/reasoning" => { /* toggle */ },
        _ => println!("Unknown command: {}", cmd),
    }
    Ok(false)
}
```

- [ ] **Step 3: Update main.rs to wire CLI and REPL**

```rust
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "shai", version, about = "AI shell agent")]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
    
    #[arg(short, long)]
    verbose: bool,
    #[arg(short, long)]
    debug: bool,
    #[arg(short, long)]
    show_reasoning: bool,
    #[arg(long)]
    provider: Option<String>,
    #[arg(long)]
    model: Option<String>,
    
    prompt: Option<String>,
}

#[derive(Subcommand)]
enum Commands {
    Config { /* config subcommands */ },
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    
    // Load config with CLI overrides
    let mut config = crate::config::Config::load()?;
    // Apply CLI flags...
    
    if let Some(Commands::Config { ... }) = cli.command {
        // Handle config subcommand
    } else if let Some(prompt) = cli.prompt {
        // One-shot mode
        crate::agent::run_agent(&prompt, &config).await?;
    } else {
        // REPL mode
        crate::repl::run_repl(&config).await?;
    }
    
    Ok(())
}
```

- [ ] **Step 4: Run integration test**

```bash
cargo test repl_test -- --nocapture
```
Expected: PASS

- [ ] **Step 5: Commit**

```bash
git add src/repl.rs src/main.rs tests/repl_test.rs
git commit -m "feat: REPL with rustyline and command handling"
```

---

### Task 15: Integration & Polish

**Files:**
- Modify: `src/main.rs` (complete)
- Create: `tests/integration.rs`
- Create: `README.md` (project root)

**Interfaces:**
- Full end-to-end working binary

- [ ] **Step 1: Write integration smoke test**

```rust
// tests/integration.rs
#[tokio::test]
async fn test_one_shot_ls() {
    let output = std::process::Command::new("cargo")
        .args(["run", "--", "list files in current directory"])
        .output()
        .expect("Failed to run shai");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Cargo.toml") || stdout.contains("src"));
}
```

- [ ] **Step 2: Run full test suite**

```bash
cargo test --all
```
Expected: All tests pass

- [ ] **Step 3: Build release binary**

```bash
cargo build --release
```

- [ ] **Step 4: Manual verification**

```bash
./target/release/shai "echo hello"
./target/release/shai -v "ls -la"
./target/release/shai  # REPL, test /help, /exit
```

- [ ] **Step 5: Create project README.md**

- [ ] **Step 6: Commit**

```bash
git add tests/integration.rs README.md
git commit -m "feat: integration tests and release build verification"
```

---

## Execution Recommendation

**Plan complete and saved to `docs/superpowers/plans/2026-11-08-shai-ai-shell-agent.md`. Please review the plan. Which execution approach would you prefer?**

- **Subagent-driven** - A fresh subagent implements each task and a fresh reviewer checks it before the next one starts, then a whole-branch review at the end. Most thorough; costs a fresh context per task and per review.
- **Native** - I implement every task myself in this session, the way this harness runs work, then one fresh reviewer on the most capable model checks the whole branch. Cheapest and fastest; no independent review until the end.

**For this plan I recommend Native, because** the tasks have strong sequential dependencies (each builds on the previous interfaces), there are 15 tasks total, and a shipped mistake in the core agent loop would require significant rework — but the plan's interfaces are precise enough that a single implementer can maintain consistency across all tasks. Does the plan capture what you want, and which approach should we use?