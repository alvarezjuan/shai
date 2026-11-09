use crate::config::{Config, ProviderConfig, RiskProviderConfig};
use crate::history::History;
use crate::provider::build_model;
use crate::risk::RiskClassifier;
use crate::tool::shell::{set_shell_config, set_shell_risk_classifier, ShellTool};
use crate::tool::{
    glob::GlobTool, grep::GrepTool, list_dir::ListDirTool, read::ReadTool, web_fetch::WebFetchTool,
    web_search::WebSearchTool,
};
use anyhow::Result;
use rig::agent::AgentBuilder;
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

    // Set global shell risk classifier and config
    set_shell_risk_classifier(risk_classifier.clone());
    set_shell_config(config.clone());

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
        .build();

    // Load history
    let history_path = PathBuf::from(&config.session.history_path);
    let mut history = History::load(&history_path, config.session.max_history_turns)?;

    // Run agent with history (rig handles tool execution automatically)
    let runner = agent.prompt(prompt).history(history.clone()).max_turns(10);

    let response = runner.await?;

    // Verify model supports tool calls
    let has_tool_calls = response.content().iter().any(|c| {
        matches!(c, rig::completion::message::AssistantContent::ToolCall(_))
    });

    if !has_tool_calls {
        // Model doesn't support tool calling - return error
        return Err(anyhow::anyhow!(
            "Model does not support tool/function calling. This model cannot execute shell commands.
Please use a model that supports OpenAI-style function calling (e.g., gpt-4o, claude-3.5-sonnet, llama3.1 with proper tool support).
"));
    }

    // Save history
    for msg in response.messages() {
        history.push(msg.clone());
    }
    History::save(&history_path, &history)?;

    Ok(response.output())
}
