# Data Flow: One Turn

Complete flow for a single agent turn from user prompt to response.

```mermaid
flowchart TD
    START([User Prompt])
    LOAD["Load History<br>.shai/session.jsonl<br>last 50 turns"]
    BUILD["Build Agent<br>Primary model + 15 tools"]
    RUN["Run Turn<br>max_turns(1)"]
    RESP["Response"]

    START --> LOAD --> BUILD --> RUN --> RESP

    RESP -- "Text only" --> PRINT["Print response<br>Save history<br>DONE"]
    RESP -- "Tool call" --> DISPATCH["Dispatch to handler"]

    DISPATCH -- "shell" --> RISK["Risk Classifier<br>LLM call → Level 1-4"]
    DISPATCH -- "read/glob/grep/..." --> EXEC_READ["Execute<br>Level 1 auto"]
    DISPATCH -- "git_commit/push" --> CONFIRM_W["Confirm y/N<br>Level 3"]
    DISPATCH -- "edit/patch" --> CONFIRM_W
    DISPATCH -- "web_search/fetch" --> EXEC_READ

    RISK -- "Level 1 Allowed" --> EXEC_SHELL["Execute sh -c"]
    RISK -- "Level 2 ReadSensitive" --> CONFIRM_R["Confirm y/N"]
    RISK -- "Level 3 WriteRecoverable" --> CONFIRM_R
    RISK -- "Level 4 Catastrophic" --> DENY["Deny: ToolResult error"]

    CONFIRM_R -- Yes --> EXEC_SHELL
    CONFIRM_R -- No --> DENY
    CONFIRM_W -- Yes --> EXEC_WRITE["Execute tool"]
    CONFIRM_W -- No --> DENY

    EXEC_SHELL --> CAPTURE["Capture stdout/stderr/exit"]
    EXEC_READ --> RESULT["ToolResult"]
    EXEC_WRITE --> RESULT
    DENY --> RESULT

    RESULT --> HISTORY_PUSH["Push to history<br>Message::tool_result"]
    HISTORY_PUSH --> NEXT_TURN["Next turn<br>Empty prompt"]
    NEXT_TURN --> RUN

    PRINT --> SAVE["Save history<br>.shai/session.jsonl"]
```

---

## Turn Phases

| Phase | Action | Component |
|-------|--------|-----------|
| **1. Load** | Read last 50 messages from `.shai/session.jsonl` | History Manager |
| **2. Build** | Create AgentBuilder with primary model + 15 tools | AgentBuilder |
| **3. Run** | Execute one turn (`max_turns(1)`) | Turn Runner |
| **4. Dispatch** | Route tool call to handler | Tool Registry |
| **5. Risk** | LLM classifies `shell` commands (Level 1-4) | Risk Classifier |
| **6. Confirm** | Prompt user for Level 2/3 | User Confirmation |
| **7. Execute** | Run command/tool, capture result | Shell Executor / Tool Handler |
| **8. History** | Append `ToolResult` as `Message::tool_result` | History Manager |
| **9. Next** | Loop with empty prompt (continuation) | Turn Runner |
| **10. Final** | Print response.text(), save history | CLI |

---

## Tool Risk Levels

| Tool | Risk Level | Confirmation |
|------|------------|--------------|
| `shell` | 1–4 (LLM) | Level 2/3: y/N, Level 4: deny |
| `read`, `glob`, `grep`, `list_dir` | 1 (Allowed) | None |
| `web_search`, `web_fetch` | 1 (Allowed) | None |
| `git_status`, `git_diff`, `git_log` | 1 (Allowed) | None |
| `git_commit`, `git_push` | 3 (WriteRecoverable) | Yes |
| `edit`, `patch` | 3 (WriteRecoverable) | Yes |
| `todo_read`, `todo_write` | 1 (Allowed) | None |

---

## History Format (JSONL)

```json
{"role":"user","content":"list files"}
{"role":"assistant","content":"","tool_calls":[{"id":"call_1","name":"shell","arguments":"{\"command\":\"ls -la\"}"}]}
{"role":"tool","tool_call_id":"call_1","content":"{\"stdout\":\"...\",\"stderr\":\"\",\"exit_code\":0}"}
{"role":"assistant","content":"Here are your files..."}
```

Each turn appends 2-4 lines: user prompt → assistant (with tool_calls) → tool result → assistant (final)