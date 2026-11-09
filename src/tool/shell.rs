use rig::tool::{Tool, ToolContext};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
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
}

pub struct ShellTool;

impl Tool for ShellTool {
    const NAME: &'static str = "shell";

    fn description(&self) -> String {
        "Execute a shell command via `sh -c` and return stdout, stderr, exit code.".to_string()
    }

    fn parameters(&self) -> serde_json::Value {
        serde_json::to_value(schemars::schema_for!(ShellArgs)).unwrap()
    }

    type Args = ShellArgs;
    type Output = ShellOutput;
    type Error = ShellError;

    async fn call(&self, _ctx: &mut ToolContext, _args: Self::Args) -> Result<Self::Output, Self::Error> {
        // STUB: Actual execution with risk check happens in AgentLoop
        Ok(ShellOutput {
            stdout: String::new(),
            stderr: String::new(),
            exit_code: 0,
        })
    }
}

pub async fn execute_command(command: &str) -> Result<ShellOutput, ShellError> {
    let mut cmd = Command::new("sh");
    cmd.arg("-c").arg(command);

    let output = timeout(Duration::from_secs(30), cmd.output())
        .await
        .map_err(|_| ShellError::Timeout(30))??;

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
        let output = execute_command("echo hello").await.unwrap();
        assert_eq!(output.stdout.trim(), "hello");
        assert_eq!(output.exit_code, 0);
    }

    #[tokio::test]
    async fn test_execute_failing_command() {
        let output = execute_command("false").await.unwrap();
        assert_ne!(output.exit_code, 0);
    }

    #[tokio::test]
    async fn test_execute_command_with_stderr() {
        let output = execute_command("sh -c 'echo stdout; echo stderr >&2'").await.unwrap();
        assert!(output.stdout.contains("stdout"));
        assert!(output.stderr.contains("stderr"));
    }
}