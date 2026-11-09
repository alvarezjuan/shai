# Superpowers Documentation

This folder contains the Superpowers methodology artifacts for the **shai** project.

---

## Contents

| Folder | Description |
|--------|-------------|
| [specs/](specs/) | Design specifications — the "what" and "why" |
| [plans/](plans/) | Implementation plans — the "how" and "when" |

---

## Specs

| File | Description | Status |
|------|-------------|--------|
| [2026-11-08-ai-shell-agent-design.md](specs/2026-11-08-ai-shell-agent-design.md) | Complete design specification for shai AI shell agent | **Approved** |

---

## Plans

| File | Description | Status |
|------|-------------|--------|
| [2026-11-08-shai-ai-shell-agent.md](plans/2026-11-08-shai-ai-shell-agent.md) | Detailed implementation plan with 15 tasks | **Ready for execution** |

---

## Superpowers Workflow

```
┌─────────────┐     ┌─────────────┐     ┌─────────────┐
│  Brainstorm │────▶│    Spec     │────▶│    Plan     │
│  (explore)  │     │  (decide)   │     │  (execute)  │
└─────────────┘     └─────────────┘     └─────────────┘
      │                   │                   │
      ▼                   ▼                   ▼
  New idea          Signed off          Task breakdown
  / problem         by stakeholder      / assignments
```

### When to Use Each

| Artifact | Purpose | Audience | Lifespan |
|----------|---------|----------|----------|
| **Brainstorm** | Explore ideas, clarify requirements | Team, stakeholders | Transient (archived after spec) |
| **Spec** | Document decisions, requirements, architecture | Team, stakeholders | Living (updated as needed) |
| **Plan** | Break into tasks, assign, track execution | Implementers | Living (updated during execution) |

---

## For This Project (shai)

| Phase | Artifact | Location |
|-------|----------|----------|
| **Brainstorm** | Completed (6 questions resolved) | Archived |
| **Spec** | AI Shell Agent Design | `specs/2026-11-08-ai-shell-agent-design.md` |
| **Plan** | Implementation Plan (15 tasks) | `plans/2026-11-08-shai-ai-shell-agent.md` |

### Current Status

- ✅ **Spec approved** — Ready for implementation
- ✅ **Plan created** — 15 tasks with interfaces and acceptance criteria
- ⏳ **Execution** — Awaiting go-ahead (native or subagent-driven)

---

## Navigation

- [Architecture Documentation](../architecture/README.md)
- [Testing Documentation](../testing/README.md)
- [Main README](../../README.md)