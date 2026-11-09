# Component Architecture

High-level system components and their relationships.

```mermaid
graph TB
    subgraph CLI["CLI Layer"]
        ONESHOT["One-shot shai prompt"]
        REPL["REPL shai prompt"]
        CONFIG_CMD["Config shai config"]
    end

    subgraph AGENT["Agent Core"]
        BUILDER["AgentBuilder preamble system prompt tools temperature max_tokens"]
        RUNNER["Turn Runner max turns history injection response parsing"]
        HISTORY["History Manager JSONL load save session jsonl max 50 turns"]
    end

    subgraph TOOLS["Tool Registry 15 v1 tools"]
        SHELL["shell Risk LLM levels 1 to 4 Exec sh dash c"]
        READ["read glob grep list dir Risk level 1 auto"]
        WEB["web search web fetch Risk level 1 auto"]
        GIT_R["git status diff log Risk level 1 auto"]
        GIT_W["git commit push Risk level 3 confirm"]
        EDIT["edit patch Risk level 3 confirm"]
        TODO["todo read write Risk level 1 auto"]
    end

    subgraph RISK["Risk Classifier"]
        RISK_MODEL["Risk Model separate config temperature zero max tokens one hundred"]
        PROMPT["Risk Prompt few shot examples JSON output levels 1 to 4"]
    end

    subgraph EXEC["Execution Layer"]
        SHELL_EXEC["Shell Executor tokio Command sh dash c capture stdout stderr exit"]
        CONFIRM["User Confirmation y N prompt Level 2,3 only"]
    end

    subgraph PROVIDERS["LLM Providers"]
        PRIMARY["Primary Model provider config gpt-4o claude llama"]
        RISK_PROV["Risk Model risk provider config gpt-4o-mini ollama"]
    end

    subgraph CONFIG["Configuration"]
        TOML["config dot toml"]
        ENV["Environment Variables"]
        DEFAULTS["Built-in Defaults"]
    end

    %% Connections
    CLI --> AGENT
    ONESHOT --> BUILDER
    REPL --> BUILDER
    CONFIG_CMD --> TOML

    BUILDER --> PRIMARY
    BUILDER --> TOOLS
    RUNNER --> HISTORY
    RUNNER --> BUILDER

    TOOLS --> RISK
    SHELL --> RISK_MODEL
    RISK_MODEL --> PROMPT
    RISK_MODEL --> RISK_PROV

    RISK --> EXEC
    RISK --> CONFIRM
    SHELL --> SHELL_EXEC
    CONFIRM --> SHELL_EXEC

    PRIMARY --> PROVIDERS
    RISK_PROV --> PROVIDERS

    TOML --> CONFIG
    ENV --> CONFIG
    DEFAULTS --> CONFIG
    CONFIG --> BUILDER
    CONFIG --> RISK_MODEL
```

---

## Component Responsibilities

| Component | Responsibility |
|-----------|----------------|
| **CLI Layer** | Parse args, dispatch to one-shot/REPL/config |
| **AgentBuilder** | Configure primary model, preamble, tools |
| **Turn Runner** | Execute one turn, parse response |
| **History Manager** | Load/save JSONL, trim to 50 turns |
| **Tool Registry** | 15 tools with fixed risk levels + shell LLM-classified |
| **Risk Classifier** | Separate LLM call, temperature zero, JSON output |
| **Shell Executor** | sh dash c via tokio, capture stdout stderr exit |
| **User Confirmation** | Blocking y N prompt for Level 2,3 |
| **Providers** | OpenAI, Anthropic, Ollama factories |
| **Configuration** | Env greater than TOML greater than Defaults priority merge |

---

## Key Design Decisions

- **Dual LLM** (ADR-006): Primary for commands, separate Risk model
- **Manual Turn Loop** (ADR-002): max turns 1 for full control
- **LLM-only Risk** (ADR-001): No deny allow patterns, few-shot prompt
- **Shell via sh dash c** (ADR-003): Supports pipes redirects builtins