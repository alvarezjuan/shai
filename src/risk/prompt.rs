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
        recent_commands.iter().rev().take(3).rev().cloned().collect::<Vec<_>>().join("\n")
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_risk_result_deserialize() {
        let json = r#"{"level": "Allowed", "reason": "read-only command"}"#;
        let result: RiskResult = serde_json::from_str(json).unwrap();
        assert_eq!(result.level, RiskLevel::Allowed);
    }

    #[test]
    fn test_risk_prompt_contains_command() {
        let prompt = build_risk_prompt("ls -la", "/home/user", &[]);
        assert!(prompt.contains("ls -la"));
        assert!(prompt.contains("/home/user"));
    }
}