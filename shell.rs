use once_cell::sync::Lazy;
use rig::tool::{Tool, ToolContext};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::sync::Mutex;
use std::time::Duration;
use thiserror::Error;
use tokio::process::Command;
use tokio::time::timeout;

#[derive(Deserialize, JsonSchema)]
pub struct ShellArgs {
    pub command: String,
    #[serde(default = "default_timeout")]
    pub timeout_secs: u64,
}

fn default_timeout() -> u64 {
    30
}

#[derive(Serialize, JsonSchema)]
pub struct ShellOutput {
    pub stdout: String,
    pub stderr: String,
    pub exit_code: i32,
}

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
    #[error("Risk evaluation failed: {0}")]
    RiskEval(String),
}

// Global risk classifier and config for shell tool
static SHELL_RISK_CLASSIFIER: Lazy<Mutex<Option<crate::risk::RiskClassifier>>> =
    Lazy::new(|| Mutex::new(None));
static SHELL_CONFIG: Lazy<Mutex<Option<crate::config::Config>>> = Lazy::new(|| Mutex::new(None));

pub fn set_shell_risk_classifier(classifier: crate::risk::RiskClassifier) {
    *SHELL_RISK_CLASSIFIER.lock().unwrap() = Some(classifier);
}

pub fn set_shell_config(config: crate::config::Config) {
    *SHELL_CONFIG.lock().unwrap() = Some(config);
}

pub struct ShellTool;

impl Tool for ShellTool {
    const NAME: &'static str = "shell";

    fn description(&self) -> String {
        "Execute a shell command via `sh -c` and return stdout, stderr, exit code. Commands are evaluated for risk before execution.".to_string()
    }

    fn parameters(&self) -> serde_json::Value {
        serde_json::to_value(schemars::schema_for!(ShellArgs)).unwrap()
    }

    type Args = ShellArgs;
    type Output = ShellOutput;
    type Error = ShellError;

    async fn call(
        &self,
        _ctx: &mut ToolContext,
        args: Self::Args,
    ) -> Result<Self::Output, Self::Error> {
        // Extract classifier and config from globals (drop guards before await)
        let classifier = {
            let guard = SHELL_RISK_CLASSIFIER.lock().unwrap();
            guard.clone()
        };
        let config = {
            let guard = SHELL_CONFIG.lock().unwrap();
            guard.clone()
        };

        if let (Some(classifier), Some(config)) = (classifier, config) {
            let cwd = std::env::current_dir()
                .map(|p| p.to_string_lossy().to_string())
                .unwrap_or_default();

            let risk = classifier
                .evaluate(&args.command, &cwd, &[])
                .await
                .map_err(|e| ShellError::RiskEval(e.to_string()))?;

            match risk.level {
                crate::risk::prompt::RiskLevel::Allowed => {
                    execute_command(&args.command, args.timeout_secs).await
                }
                crate::risk::prompt::RiskLevel::ReadSensitive
                | crate::risk::prompt::RiskLevel::WriteRecoverable => {
                    if prompt_user(&args.command, &risk, &config).await? {
                        execute_command(&args.command, args.timeout_secs).await
                    } else {
                        Err(ShellError::UserDenied("User denied execution".into()))
                    }
                }
                crate::risk::prompt::RiskLevel::Catastrophic => Err(ShellError::Denied(format!(
                    "Catastrophic risk: {}",
                    risk.reason
                ))),
            }
        } else {
            // Fallback: execute without risk check (should not happen in normal operation)
            execute_command(&args.command, args.timeout_secs).await
        }
    }
}

async fn prompt_user(
    command: &str,
    risk: &crate::risk::prompt::RiskResult,
    config: &crate::config::Config,
) -> Result<bool, ShellError> {
    if !config.display.verbose && !config.display.debug {
        println!(
            "\n⚠️  Risk Level: {:?} - {}\nCommand: {}\nExecute? [y/N]: ",
            risk.level, risk.reason, command
        );
    } else {
        println!(
            "\n⚠️  Risk Assessment:\n  Level: {:?}\n  Reason: {}\n  Command: {}\nExecute? [y/N]: ",
            risk.level, risk.reason, command
        );
    }

    use std::io::{self, Write};
    io::stdout().flush().map_err(ShellError::Exec)?;

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .map_err(ShellError::Exec)?;
    Ok(input.trim().eq_ignore_ascii_case("y"))
}

pub async fn execute_command(command: &str, timeout_secs: u64) -> Result<ShellOutput, ShellError> {
    let mut cmd = Command::new("sh");
    cmd.arg("-c").arg(command);

    let output = timeout(Duration::from_secs(timeout_secs), cmd.output())
        .await
        .map_err(|_| ShellError::Timeout(timeout_secs))??;

    Ok(ShellOutput {
        stdout: String::from_utf8_lossy(&output.stdout).to_string(),
        stderr: String::from_utf8_lossy(&output.stderr).to_string(),
        exit_code: output.status.code().unwrap_or(-1),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_execute_simple_command() {
        let output = execute_command("echo hello", 30).await.unwrap();
        assert_eq!(output.stdout.trim(), "hello");
        assert_eq!(output.exit_code, 0);
    }

    #[tokio::test]
    async fn test_execute_failing_command() {
        let output = execute_command("false", 30).await.unwrap();
        assert_ne!(output.exit_code, 0);
    }

    #[tokio::test]
    async fn test_execute_command_with_stderr() {
        let output = execute_command("sh -c 'echo stdout; echo stderr >&2'", 30)
            .await
            .unwrap();
        assert!(output.stdout.contains("stdout"));
        assert!(output.stderr.contains("stderr"));
    }
}
