---
layout: default
title: Tools Reference
nav_order: 5
---

# Built-in Tools (7)

shai includes 7 built-in tools across 3 categories.

## Execute

### `shell`

Run shell commands with automatic risk classification.

**Parameters:**
```json
{
  "command": "string",
  "description": "string (optional)"
}
```

**Risk:** Dynamic (1-4) — classified by risk LLM

**Examples:**
```bash
shai "list files"                    # Level 1: ls -la
shai "delete temp files"             # Level 3: rm *.tmp (confirm)
shai "show disk usage"               # Level 1: df -h
shai "run cargo test"                # Level 1: cargo test
```

**Confirmation:** Level 2/3 prompt user, Level 4 denied

---

## File Read

### `read`

Read file contents.

**Parameters:**
```json
{
  "path": "string",
  "offset": "number (optional)",
  "limit": "number (optional)"
}
```

**Risk:** Level 1 (Auto-allowed)

**Path:** Must be within current working directory

**Examples:**
```bash
shai "show me Cargo.toml"
shai "read src/main.rs lines 10-50"
```

---

### `glob`

Find files by pattern.

**Parameters:**
```json
{
  "pattern": "string",
  "path": "string (optional)"
}
```

**Risk:** Level 1 (Auto-allowed)

**Pattern:** Glob syntax (`**/*.rs`, `src/**/*.toml`, etc.)

**Examples:**
```bash
shai "find all Rust files"
shai "find test files in tests/"
```

---

### `grep`

Search file contents.

**Parameters:**
```json
{
  "pattern": "string",
  "path": "string (optional)",
  "include": "string (optional)"
}
```

**Risk:** Level 1 (Auto-allowed)

**Examples:**
```bash
shai "search for 'TODO' in src/"
shai "find 'unwrap()' calls"
shai "grep 'async fn' in *.rs"
```

---

### `list_dir`

List directory contents.

**Parameters:**
```json
{
  "path": "string (optional)"
}
```

**Risk:** Level 1 (Auto-allowed)

**Examples:**
```bash
shai "list src/ directory"
shai "show project root contents"
```

---

## Web

### `web_search`

Search DuckDuckGo.

**Parameters:**
```json
{
  "query": "string",
  "max_results": "number (optional, default: 10)"
}
```

**Risk:** Level 1 (Auto-allowed)

**Examples:**
```bash
shai "search for Rust async best practices"
shai "search for 'tokio tutorial' max 5 results"
```

---

### `web_fetch`

Fetch and extract content from URL.

**Parameters:**
```json
{
  "url": "string",
  "max_length": "number (optional, default: 5000)"
}
```

**Risk:** Level 1 (Auto-allowed)

**Examples:**
```bash
shai "fetch https://doc.rust-lang.org/book/"
shai "get content from https://github.com/rust-lang/rust"
```

---

## Tool Invocation Flow

```
LLM Response
    ↓
Tool Call Detected
    ↓
Risk Classifier (for shell) / Auto-Level 1 (for others)
    ↓
Level 1: Execute immediately
Level 2/3: Prompt user [y/N]
Level 4: Deny
    ↓
Execute → ToolResult
    ↓
Feed back to LLM for next turn
```

## Risk Levels by Tool

| Tool | Default Risk | Confirmation |
|------|-------------|--------------|
| `shell` | Dynamic (1-4) | L2/3: y/N, L4: deny |
| `read` | 1 (Allowed) | None |
| `glob` | 1 (Allowed) | None |
| `grep` | 1 (Allowed) | None |
| `list_dir` | 1 (Allowed) | None |
| `web_search` | 1 (Allowed) | None |
| `web_fetch` | 1 (Allowed) | None |

## Custom Tools (Future)

v2.x will support a plugin system for custom tools. For now, all tools are built-in.

See [Architecture](/architecture/) for how to add tools to the codebase.
