# Security Policy

## Supported Versions

We take security seriously. The following versions of shai are currently supported with security updates:

| Version | Supported |
|---------|-----------|
| 0.1.x   | ✅ Yes    |

As a pre-1.0 project, only the latest minor version receives security updates.

## Reporting a Vulnerability

We appreciate responsible disclosure of security vulnerabilities. If you discover a security issue, please report it responsibly.

### How to Report

**Do not** create a public GitHub issue for security vulnerabilities.

Instead, email us at: **security@shai.example.com**

Include the following information:

1. **Description** of the vulnerability
2. **Steps to reproduce** (if applicable)
3. **Impact** assessment (what could an attacker achieve?)
4. **Affected versions** (if known)
5. **Suggested fix** (if you have one)

### What to Expect

- **Acknowledgment** within 48 hours
- **Initial assessment** within 5 business days
- **Regular updates** on progress
- **Credit** in the security advisory (unless you prefer anonymity)

## Security Architecture

### Risk Classification

shai's core security feature is its four-tier risk classifier:

| Level | Name | Action |
|-------|------|--------|
| 1 | Allowed | Execute immediately |
| 2 | ReadSensitive | Confirm (y/N) |
| 3 | WriteRecoverable | Confirm (y/N) |
| 4 | Catastrophic | Deny (no prompt) |

### Key Security Principles

1. **No unattended execution** — Every shell command requires risk assessment
2. **Human-in-the-loop** — Destructive operations require explicit user confirmation
3. **LLM-only risk evaluation** — No hardcoded deny/allow lists that can be bypassed
4. **Path validation** — File operations restricted to current working directory
5. **No secret storage** — API keys never written to history or logs

### Threat Model

| Threat | Mitigation |
|--------|------------|
| Command injection | LLM risk classifier evaluates every command; user confirms sensitive ops |
| Path traversal | File tools validate paths are within CWD |
| Secret leakage | Risk classifier identifies sensitive reads; API keys only via env vars |
| Unauthorized execution | Level 4 commands denied; Level 2/3 require explicit y/N |
| History exposure | No secrets in history; risk classifier detects sensitive data |

## Known Limitations

### Pre-1.0 Software

shai is currently pre-1.0. The API and security model may change. Use in production at your own risk.

### LLM-Based Risk Classification

The risk classifier uses an LLM, which means:

- **Non-deterministic** — Same command may get different risk levels
- **Prompt injection risk** — Malicious commands could attempt to manipulate the classifier
- **Model dependency** — Security depends on the risk model's training and alignment

### Git Operations

Git write tools (`git_commit`, `git_push`) are currently stubbed and not functional. When implemented, they will follow the same risk/confirmation flow.

### Shell Execution

Commands execute via `sh -c` which supports:
- Pipes, redirects, shell builtins
- Variable expansion, globbing
- This power requires careful risk evaluation

## Secure Configuration

### Recommended Practices

1. **Use environment variables** for API keys, not config files:
   ```bash
   export OPENAI_API_KEY="sk-..."
   ```

2. **Restrict config file permissions**:
   ```bash
   chmod 600 ~/.config/shai/config.toml
   ```

3. **Use a dedicated risk model** (smaller/faster):
   ```toml
   [risk_provider]
   type = "openai"
   model = "gpt-4o-mini"
   temperature = 0.0
   ```

4. **Run with minimal privileges** — Never as root

### Ollama Local Deployment

For fully local operation with no external API calls:

```bash
# Start Ollama
ollama serve

# Pull models
ollama pull llama3.1
ollama pull nomic-embed-text  # for future RAG

# Configure shai
export SHAI_PROVIDER=ollama
export SHAI_MODEL=llama3.1
export SHAI_RISK_PROVIDER=ollama
export SHAI_RISK_MODEL=llama3.1
```

## Vulnerability Disclosure Timeline

| Phase | Timeline |
|-------|----------|
| Report received | Day 0 |
| Acknowledgment | ≤ 48 hours |
| Initial assessment | ≤ 5 business days |
| Fix development | ≤ 30 days (varies by severity) |
| Release & advisory | ≤ 7 days after fix |
| Public disclosure | After release |

## Security Advisories

Security advisories will be published via:

- GitHub Security Advisories
- Release notes
- Email to security mailing list (if established)

## Contact

For security concerns, contact: **security@shai.example.com**

For general questions, use [GitHub Discussions](https://github.com/yourusername/shai/discussions).

---

*This policy is based on industry best practices and may be updated as the project evolves.*