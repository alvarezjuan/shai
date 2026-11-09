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

use crate::config::Config;
use crate::risk::RiskClassifier;
use anyhow::Result;
use rig::completion::message::ToolCall;
use rig::tool::{Tool, ToolExecutionError, ToolResult, ToolContext};

pub struct ToolRegistry;

impl ToolRegistry {
    pub async fn dispatch(
        tool_call: &ToolCall,
        risk_classifier: &RiskClassifier,
        config: &Config,
    ) -> Result<ToolResult> {
        // Get recent commands for risk context
        let recent_commands = Self::get_recent_commands();

        match tool_call.function.name.as_str() {
            "shell" => Self::handle_shell(tool_call, risk_classifier, config, &recent_commands).await,
            "read" => Self::handle_read(tool_call).await,
            "glob" => Self::handle_glob(tool_call).await,
            "grep" => Self::handle_grep(tool_call).await,
            "list_dir" => Self::handle_list_dir(tool_call).await,
            "web_search" => Self::handle_web_search(tool_call).await,
            "web_fetch" => Self::handle_web_fetch(tool_call).await,
            "git_status" => Self::handle_git_status(tool_call).await,
            "git_diff" => Self::handle_git_diff(tool_call).await,
            "git_log" => Self::handle_git_log(tool_call).await,
            "git_commit" => Self::handle_git_commit(tool_call, config).await,
            "git_push" => Self::handle_git_push(tool_call, config).await,
            "edit" => Self::handle_edit(tool_call, config).await,
            "patch" => Self::handle_patch(tool_call, config).await,
            "todo_read" => Self::handle_todo_read(tool_call).await,
            "todo_write" => Self::handle_todo_write(tool_call).await,
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
        let args: shell::ShellArgs = serde_json::from_value(serde_json::Value::Object(tool_call.function.arguments.clone()))?;

        // Get current working directory
        let cwd = std::env::current_dir()
            .map(|p| p.to_string_lossy().to_string())
            .unwrap_or_default();

        // Run risk classification
        let risk = risk_classifier.evaluate(&args.command, &cwd, recent_commands).await?;

        let result = match risk.level {
            crate::risk::prompt::RiskLevel::Allowed => {
                let output = shell::execute_command(&args.command).await?;
                ToolResult::success(serde_json::to_string(&output)?.into())
            }
            crate::risk::prompt::RiskLevel::ReadSensitive | crate::risk::prompt::RiskLevel::WriteRecoverable => {
                if Self::prompt_user(&args.command, &risk, config).await? {
                    let output = shell::execute_command(&args.command).await?;
                    ToolResult::success(serde_json::to_string(&output)?.into())
                } else {
                    ToolResult::failed(ToolExecutionError::invalid_args("User denied execution"))
                }
            }
            crate::risk::prompt::RiskLevel::Catastrophic => {
                ToolResult::failed(ToolExecutionError::invalid_args(format!("Denied: catastrophic risk - {}", risk.reason)))
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
        let args: read::ReadArgs = serde_json::from_value(serde_json::Value::Object(tool_call.function.arguments.clone()))?;
        let tool = read::ReadTool;
        let mut ctx = ToolContext::default();
        let result = tool.call(&mut ctx, args).await?;
        Ok(ToolResult::success(serde_json::to_string(&result)?.into()))
    }

    async fn handle_glob(tool_call: &ToolCall) -> Result<ToolResult> {
        let args: glob::GlobArgs = serde_json::from_value(serde_json::Value::Object(tool_call.function.arguments.clone()))?;
        let tool = glob::GlobTool;
        let mut ctx = ToolContext::default();
        let result = tool.call(&mut ctx, args).await?;
        Ok(ToolResult::success(serde_json::to_string(&result)?.into()))
    }

    async fn handle_grep(tool_call: &ToolCall) -> Result<ToolResult> {
        let args: grep::GrepArgs = serde_json::from_value(serde_json::Value::Object(tool_call.function.arguments.clone()))?;
        let tool = grep::GrepTool;
        let mut ctx = ToolContext::default();
        let result = tool.call(&mut ctx, args).await?;
        Ok(ToolResult::success(serde_json::to_string(&result)?.into()))
    }

    async fn handle_list_dir(tool_call: &ToolCall) -> Result<ToolResult> {
        let args: list_dir::ListDirArgs = serde_json::from_value(serde_json::Value::Object(tool_call.function.arguments.clone()))?;
        let tool = list_dir::ListDirTool;
        let mut ctx = ToolContext::default();
        let result = tool.call(&mut ctx, args).await?;
        Ok(ToolResult::success(serde_json::to_string(&result)?.into()))
    }

    async fn handle_web_search(tool_call: &ToolCall) -> Result<ToolResult> {
        let args: web_search::WebSearchArgs = serde_json::from_value(serde_json::Value::Object(tool_call.function.arguments.clone()))?;
        let tool = web_search::WebSearchTool;
        let mut ctx = ToolContext::default();
        let result = tool.call(&mut ctx, args).await?;
        Ok(ToolResult::success(serde_json::to_string(&result)?.into()))
    }

    async fn handle_web_fetch(tool_call: &ToolCall) -> Result<ToolResult> {
        let args: web_fetch::WebFetchArgs = serde_json::from_value(serde_json::Value::Object(tool_call.function.arguments.clone()))?;
        let tool = web_fetch::WebFetchTool;
        let mut ctx = ToolContext::default();
        let result = tool.call(&mut ctx, args).await?;
        Ok(ToolResult::success(serde_json::to_string(&result)?.into()))
    }

    async fn handle_git_status(tool_call: &ToolCall) -> Result<ToolResult> {
        let args: git::GitStatusArgs = serde_json::from_value(serde_json::Value::Object(tool_call.function.arguments.clone()))?;
        let tool = git::GitStatusTool;
        let mut ctx = ToolContext::default();
        let result = tool.call(&mut ctx, args).await?;
        Ok(ToolResult::success(serde_json::to_string(&result)?.into()))
    }

    async fn handle_git_diff(tool_call: &ToolCall) -> Result<ToolResult> {
        let args: git::GitDiffArgs = serde_json::from_value(serde_json::Value::Object(tool_call.function.arguments.clone()))?;
        let tool = git::GitDiffTool;
        let mut ctx = ToolContext::default();
        let result = tool.call(&mut ctx, args).await?;
        Ok(ToolResult::success(serde_json::to_string(&result)?.into()))
    }

    async fn handle_git_log(tool_call: &ToolCall) -> Result<ToolResult> {
        let args: git::GitLogArgs = serde_json::from_value(serde_json::Value::Object(tool_call.function.arguments.clone()))?;
        let tool = git::GitLogTool;
        let mut ctx = ToolContext::default();
        let result = tool.call(&mut ctx, args).await?;
        Ok(ToolResult::success(serde_json::to_string(&result)?.into()))
    }

    // Level 3 tools - require confirmation
    async fn handle_git_commit(tool_call: &ToolCall, config: &Config) -> Result<ToolResult> {
        let args: git::GitCommitArgs = serde_json::from_value(serde_json::Value::Object(tool_call.function.arguments.clone()))?;

        if Self::prompt_user_write("git commit", &args.message, config).await? {
            let tool = git::GitCommitTool;
            let mut ctx = ToolContext::default();
            let result = tool.call(&mut ctx, args).await?;
            Ok(ToolResult::success(serde_json::to_string(&result)?.into()))
        } else {
            Ok(ToolResult::failed(ToolExecutionError::invalid_args("User denied git commit")))
        }
    }

    async fn handle_git_push(tool_call: &ToolCall, config: &Config) -> Result<ToolResult> {
        let args: git::GitPushArgs = serde_json::from_value(serde_json::Value::Object(tool_call.function.arguments.clone()))?;

        if Self::prompt_user_write("git push", &format!("{:?}", args), config).await? {
            let tool = git::GitPushTool;
            let mut ctx = ToolContext::default();
            let result = tool.call(&mut ctx, args).await?;
            Ok(ToolResult::success(serde_json::to_string(&result)?.into()))
        } else {
            Ok(ToolResult::failed(ToolExecutionError::invalid_args("User denied git push")))
        }
    }

    async fn handle_edit(tool_call: &ToolCall, config: &Config) -> Result<ToolResult> {
        let args: edit::EditArgs = serde_json::from_value(serde_json::Value::Object(tool_call.function.arguments.clone()))?;

        if Self::prompt_user_write("edit file", &format!("{} -> {}", args.old_string, args.new_string), config).await? {
            let tool = edit::EditTool;
            let mut ctx = ToolContext::default();
            let result = tool.call(&mut ctx, args).await?;
            Ok(ToolResult::success(serde_json::to_string(&result)?.into()))
        } else {
            Ok(ToolResult::failed(ToolExecutionError::invalid_args("User denied file edit")))
        }
    }

    async fn handle_patch(tool_call: &ToolCall, config: &Config) -> Result<ToolResult> {
        let args: edit::PatchArgs = serde_json::from_value(serde_json::Value::Object(tool_call.function.arguments.clone()))?;

        if Self::prompt_user_write("apply patch", &args.patch, config).await? {
            let tool = edit::PatchTool;
            let mut ctx = ToolContext::default();
            let result = tool.call(&mut ctx, args).await?;
            Ok(ToolResult::success(serde_json::to_string(&result)?.into()))
        } else {
            Ok(ToolResult::failed(ToolExecutionError::invalid_args("User denied patch")))
        }
    }

    async fn prompt_user_write(action: &str, details: &str, config: &Config) -> Result<bool> {
        if !config.display.verbose && !config.display.debug {
            println!("\n✏️  {}: {}\nConfirm? [y/N]: ", action, details);
        } else {
            println!("\n✏️  Write Operation:\n  Action: {}\n  Details: {}\nConfirm? [y/N]: ", action, details);
        }

        use std::io::{self, Write};
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        Ok(input.trim().eq_ignore_ascii_case("y"))
    }

    async fn handle_todo_read(tool_call: &ToolCall) -> Result<ToolResult> {
        let args: todo::TodoReadArgs = serde_json::from_value(serde_json::Value::Object(tool_call.function.arguments.clone()))?;
        let tool = todo::TodoReadTool;
        let mut ctx = ToolContext::default();
        let result = tool.call(&mut ctx, args).await?;
        Ok(ToolResult::success(serde_json::to_string(&result)?.into()))
    }

    async fn handle_todo_write(tool_call: &ToolCall) -> Result<ToolResult> {
        let args: todo::TodoWriteArgs = serde_json::from_value(serde_json::Value::Object(tool_call.function.arguments.clone()))?;
        let tool = todo::TodoWriteTool;
        let mut ctx = ToolContext::default();
        let result = tool.call(&mut ctx, args).await?;
        Ok(ToolResult::success(serde_json::to_string(&result)?.into()))
    }
}