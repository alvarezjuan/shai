pub mod glob;
pub mod grep;
pub mod list_dir;
pub mod read;
pub mod shell;
pub mod web_fetch;
pub mod web_search;

use crate::config::Config;
use crate::risk::RiskClassifier;
use anyhow::Result;
use rig::completion::message::ToolCall;
use rig::tool::{Tool, ToolContext, ToolExecutionError, ToolResult};

#[allow(dead_code)]
pub struct ToolRegistry;

#[allow(dead_code)]
impl ToolRegistry {
    pub async fn dispatch(
        tool_call: &ToolCall,
        risk_classifier: &RiskClassifier,
        config: &Config,
    ) -> Result<ToolResult> {
        // Get recent commands for risk context
        let recent_commands = Self::get_recent_commands();

        match tool_call.function.name.as_str() {
            "shell" => {
                Self::handle_shell(tool_call, risk_classifier, config, &recent_commands).await
            }
            "read" => Self::handle_read(tool_call).await,
            "glob" => Self::handle_glob(tool_call).await,
            "grep" => Self::handle_grep(tool_call).await,
            "list_dir" => Self::handle_list_dir(tool_call).await,
            "web_search" => Self::handle_web_search(tool_call).await,
            "web_fetch" => Self::handle_web_fetch(tool_call).await,
            _ => Err(anyhow::anyhow!("Unknown tool: {}", tool_call.function.name)),
        }
    }

    fn get_recent_commands() -> Vec<String> {
        // TODO: Extract from history or maintain a separate command history
        Vec::new()
    }

    async fn handle_shell(
        tool_call: &ToolCall,
        risk_classifier: &RiskClassifier,
        config: &Config,
        recent_commands: &[String],
    ) -> Result<ToolResult> {
        println!("[DEBUG shell] handle_shell called");
        let args: shell::ShellArgs = serde_json::from_value(serde_json::Value::Object(
            tool_call.function.arguments.clone(),
        ))?;

        // Get current working directory
        let cwd = std::env::current_dir()
            .map(|p| p.to_string_lossy().to_string())
            .unwrap_or_default();

        // Run risk classification
        let risk = risk_classifier
            .evaluate(&args.command, &cwd, recent_commands)
            .await?;
        eprintln!(
            "[DEBUG shell] risk level: {:?}, reason: {}",
            risk.level, risk.reason
        );

        let result = match risk.level {
            crate::risk::prompt::RiskLevel::Allowed => {
                let output = shell::execute_command(&args.command, args.timeout_secs).await?;
                eprintln!(
                    "[DEBUG shell] command: {:?}, stdout: {:?}, stderr: {:?}, exit_code: {}",
                    args.command, output.stdout, output.stderr, output.exit_code
                );
                ToolResult::success(serde_json::to_string(&output)?.into())
            }
            crate::risk::prompt::RiskLevel::ReadSensitive
            | crate::risk::prompt::RiskLevel::WriteRecoverable => {
                if Self::prompt_user(&args.command, &risk, config).await? {
                    let output = shell::execute_command(&args.command, args.timeout_secs).await?;
                    ToolResult::success(serde_json::to_string(&output)?.into())
                } else {
                    ToolResult::failed(ToolExecutionError::invalid_args("User denied execution"))
                }
            }
            crate::risk::prompt::RiskLevel::Catastrophic => {
                ToolResult::failed(ToolExecutionError::invalid_args(format!(
                    "Denied: catastrophic risk - {}",
                    risk.reason
                )))
            }
        };

        Ok(result)
    }

    async fn prompt_user(
        command: &str,
        risk: &crate::risk::prompt::RiskResult,
        config: &Config,
    ) -> Result<bool> {
        if !config.display.verbose && !config.display.debug {
            // Simple prompt
            println!(
                "\n⚠️  Risk Level: {:?} - {}\nCommand: {}\nExecute? [y/N]: ",
                risk.level, risk.reason, command
            );
        } else {
            // Verbose prompt with more details
            println!(
                "\n⚠️  Risk Assessment:\n  Level: {:?}\n  Reason: {}\n  Command: {}\nExecute? [y/N]: ",
                risk.level, risk.reason, command
            );
        }

        use std::io::{self, Write};
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        Ok(input.trim().eq_ignore_ascii_case("y"))
    }

    // Level 1 tools - auto-execute
    async fn handle_read(tool_call: &ToolCall) -> Result<ToolResult> {
        let args: read::ReadArgs = serde_json::from_value(serde_json::Value::Object(
            tool_call.function.arguments.clone(),
        ))?;
        let tool = read::ReadTool;
        let mut ctx = ToolContext::default();
        let result = tool.call(&mut ctx, args).await?;
        Ok(ToolResult::success(serde_json::to_string(&result)?.into()))
    }

    async fn handle_glob(tool_call: &ToolCall) -> Result<ToolResult> {
        let args: glob::GlobArgs = serde_json::from_value(serde_json::Value::Object(
            tool_call.function.arguments.clone(),
        ))?;
        let tool = glob::GlobTool;
        let mut ctx = ToolContext::default();
        let result = tool.call(&mut ctx, args).await?;
        Ok(ToolResult::success(serde_json::to_string(&result)?.into()))
    }

    async fn handle_grep(tool_call: &ToolCall) -> Result<ToolResult> {
        let args: grep::GrepArgs = serde_json::from_value(serde_json::Value::Object(
            tool_call.function.arguments.clone(),
        ))?;
        let tool = grep::GrepTool;
        let mut ctx = ToolContext::default();
        let result = tool.call(&mut ctx, args).await?;
        Ok(ToolResult::success(serde_json::to_string(&result)?.into()))
    }

    async fn handle_list_dir(tool_call: &ToolCall) -> Result<ToolResult> {
        let args: list_dir::ListDirArgs = serde_json::from_value(serde_json::Value::Object(
            tool_call.function.arguments.clone(),
        ))?;
        let tool = list_dir::ListDirTool;
        let mut ctx = ToolContext::default();
        let result = tool.call(&mut ctx, args).await?;
        Ok(ToolResult::success(serde_json::to_string(&result)?.into()))
    }

    async fn handle_web_search(tool_call: &ToolCall) -> Result<ToolResult> {
        let args: web_search::WebSearchArgs = serde_json::from_value(serde_json::Value::Object(
            tool_call.function.arguments.clone(),
        ))?;
        let tool = web_search::WebSearchTool;
        let mut ctx = ToolContext::default();
        let result = tool.call(&mut ctx, args).await?;
        Ok(ToolResult::success(serde_json::to_string(&result)?.into()))
    }

    async fn handle_web_fetch(tool_call: &ToolCall) -> Result<ToolResult> {
        let args: web_fetch::WebFetchArgs = serde_json::from_value(serde_json::Value::Object(
            tool_call.function.arguments.clone(),
        ))?;
        let tool = web_fetch::WebFetchTool;
        let mut ctx = ToolContext::default();
        let result = tool.call(&mut ctx, args).await?;
        Ok(ToolResult::success(serde_json::to_string(&result)?.into()))
    }
}
