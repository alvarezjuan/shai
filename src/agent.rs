use crate::config::{Config, ProviderConfig, RiskProviderConfig};
use crate::history::History;
use crate::provider::build_model;
use crate::risk::RiskClassifier;
use crate::tool::ToolRegistry;
use crate::tool::{shell::ShellTool, read::ReadTool, glob::GlobTool, grep::GrepTool, list_dir::ListDirTool, web_search::WebSearchTool, web_fetch::WebFetchTool, git::GitStatusTool, git::GitDiffTool, git::GitLogTool, git::GitCommitTool, git::GitPushTool, edit::EditTool, edit::PatchTool, todo::TodoReadTool, todo::TodoWriteTool};
use anyhow::Result;
use rig::agent::AgentBuilder;
use rig::completion::{Message, message::{ToolCall, AssistantContent}};
use rig::driver::DynModel;
use rig::operation::Completion;
use std::path::PathBuf;

const PREAMBLE: &str = r#"
You are shai, an AI assistant that executes shell commands to accomplish tasks.
Use the `shell` tool to run commands. The system will evaluate each command for risk
and may ask the user for confirmation before execution.

**Before running commands:**
- Explain what you're doing and why
- If you modify a user's command, explain the change and reason

**After commands execute:**
- Summarize the result (stdout, stderr, exit code)
- Explain what the output means for the task
- Note any errors or unexpected behavior
- Decide next steps based on results

Always be clear and concise in your explanations.
"#;

fn convert_risk_config(config: &RiskProviderConfig) -> ProviderConfig {
    ProviderConfig {
        r#type: config.r#type.clone(),
        model: config.model.clone(),
        base_url: config.base_url.clone(),
        api_key: config.api_key.clone(),
        temperature: config.temperature,
        max_tokens: config.max_tokens,
    }
}

pub async fn run_agent(prompt: &str, config: &Config) -> Result<String> {
    // Build primary model
    let primary_model = build_model(&config.provider)?;

    // Build risk model (fallback to primary if not configured)
    let risk_model = if let Some(risk_config) = &config.risk_provider {
        build_model(&convert_risk_config(risk_config))?
    } else {
        primary_model.clone()
    };

    let risk_classifier = RiskClassifier::new(risk_model);

    // Build agent with primary model and all tools
    let agent = AgentBuilder::new(primary_model)
        .name("shai")
        .preamble(PREAMBLE)
        .temperature(config.provider.temperature as f64)
        .max_tokens(config.provider.max_tokens as u64)
        .tool(ShellTool)
        .tool(ReadTool)
        .tool(GlobTool)
        .tool(GrepTool)
        .tool(ListDirTool)
        .tool(WebSearchTool)
        .tool(WebFetchTool)
        .tool(GitStatusTool)
        .tool(GitDiffTool)
        .tool(GitLogTool)
        .tool(GitCommitTool)
        .tool(GitPushTool)
        .tool(EditTool)
        .tool(PatchTool)
        .tool(TodoReadTool)
        .tool(TodoWriteTool)
        .build();

    // Load history
    let history_path = PathBuf::from(&config.session.history_path);
    let mut history = History::load(&history_path, config.session.max_history_turns)?;

    let mut user_prompt = prompt.to_string();

    loop {
        // Run exactly ONE turn
        let runner = agent
            .prompt(user_prompt.clone())
            .history(history.clone())
            .max_turns(1);

        let response = runner.await?;

        // Debug output
        if config.display.debug {
            println!("[DEBUG] Response: {:?}", response);
            // Extract tool calls from response content
            for content in response.content() {
                if let AssistantContent::ToolCall(tc) = content {
                    println!("[DEBUG] Tool call: {} with args: {:?}", tc.function.name, tc.function.arguments);
                }
            }
        }

        // Check for tool calls in response content
        let tool_call = response.content().iter().find_map(|c| {
            if let AssistantContent::ToolCall(tc) = c {
                Some(tc)
            } else {
                None
            }
        });

        if let Some(tool_call) = tool_call {
            let tool_result = ToolRegistry::dispatch(tool_call, &risk_classifier, config).await?;

            // Verbose output
            if config.display.verbose {
                if let Some(text) = tool_result.output().as_text() {
                    println!("[TOOL RESULT] {}", text);
                }
            }

            // Feed result back
            history.push(Message::tool_result(
                tool_call.id.clone(),
                tool_call.function.name.clone(),
                tool_result.output().as_text().unwrap_or_default(),
            ));
            user_prompt = String::new();
            continue;
        }

        // Final response
        let final_text = response.output();
        println!("{}", final_text);

        // Save history
        for msg in response.messages() {
            history.push(msg.clone());
        }
        History::save(&history_path, &history)?;

        return Ok(final_text);
    }
}