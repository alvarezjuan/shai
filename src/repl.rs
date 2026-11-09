use crate::agent::run_agent;
use crate::config::Config;
use crate::history::History;
use crate::risk::RiskClassifier;
use crate::provider::build_model;
use anyhow::Result;
use rig::completion::Message;
use rustyline::DefaultEditor;
use std::path::PathBuf;

pub async fn run_repl(config: &Config) -> Result<()> {
    let mut rl = DefaultEditor::new()?;
    let history_path = dirs::data_dir()
        .unwrap_or_else(|| std::path::PathBuf::from("."))
        .join("shai")
        .join("repl_history.txt");

    // Create history directory if it doesn't exist
    if let Some(parent) = history_path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    
    let _ = rl.load_history(&history_path);

    println!("shai REPL. Type /help for commands, /exit to quit.");

    loop {
        let readline = rl.readline("shai> ");
        match readline {
            Ok(line) => {
                let line = line.trim();
                if line.is_empty() {
                    continue;
                }

                if line.starts_with('/') {
                    if handle_repl_command(line, config, &mut rl, &history_path).await? {
                        break; // exit
                    }
                } else {
                    let _ = run_agent(line, config).await;
                }
                let _ = rl.add_history_entry(line);
            }
            Err(rustyline::error::ReadlineError::Interrupted) => {
                println!("\nUse /exit to quit");
            }
            Err(rustyline::error::ReadlineError::Eof) => {
                break;
            }
            Err(err) => {
                eprintln!("Error: {}", err);
                break;
            }
        }
    }

    rl.save_history(&history_path)?;
    Ok(())
}

async fn handle_repl_command(
    cmd: &str,
    config: &Config,
    _rl: &mut DefaultEditor,
    _history_path: &std::path::Path,
) -> Result<bool> {
    let parts: Vec<&str> = cmd.split_whitespace().collect();
    let command = parts[0];

    match command {
        "/exit" | "/quit" => return Ok(true),
        "/help" => print_help(),
        "/history" => show_history(config, parts.get(1).map(|s| *s)).await?,
        "/clear" => clear_history(config)?,
        "/config" => println!("{:#?}", config),
        "/risk" => {
            if parts.len() > 1 {
                dry_run_risk(config, &parts[1..].join(" ")).await?;
            } else {
                println!("Usage: /risk <command>");
            }
        }
        "/verbose" => {
            // Toggle verbose - we can't modify config directly here since it's &Config
            println!("Use -v flag when starting shai to enable verbose mode");
        }
        "/debug" => {
            println!("Use -d flag when starting shai to enable debug mode");
        }
        "/reasoning" => {
            println!("Use -r flag when starting shai to enable reasoning display");
        }
        _ => println!("Unknown command: {}. Type /help for available commands.", command),
    }
    Ok(false)
}

fn print_help() {
    println!(
        r#"
Available commands:
  /help          - Show this help
  /exit, /quit   - Save history and exit
  /history [N]   - Show last N turns (default: 10)
  /clear         - Clear history file
  /config        - Show current configuration
  /risk <cmd>    - Dry-run risk classification for a command
  /verbose       - Toggle verbose mode (show full tool output)
  /debug         - Toggle debug mode (show tool calls, risk levels)
  /reasoning     - Toggle reasoning display (if model supports it)
"#
    );
}

async fn show_history(config: &Config, n: Option<&str>) -> Result<()> {
    let limit = n.and_then(|s| s.parse::<usize>().ok()).unwrap_or(10);
    let history_path = PathBuf::from(&config.session.history_path);
    let messages = History::load(&history_path, limit)?;

    for (i, msg) in messages.iter().enumerate() {
        let role = match msg {
            rig::completion::Message::User { .. } => "User",
            rig::completion::Message::Assistant { .. } => "Assistant",
            rig::completion::Message::System { .. } => "System",
        };
        let content = match msg {
            rig::completion::Message::User { content } => {
                content.iter().map(|c| format!("{:?}", c)).collect::<Vec<_>>().join(" ")
            }
            rig::completion::Message::Assistant(turn) => {
                turn.content.iter().map(|c| format!("{:?}", c)).collect::<Vec<_>>().join(" ")
            }
            rig::completion::Message::System { content } => content.clone(),
        };
        let preview = if content.len() > 100 {
            format!("{}...", &content[..100])
        } else {
            content
        };
        println!("{}: {}: {}", i + 1, role, preview);
    }
    Ok(())
}

fn clear_history(config: &Config) -> Result<()> {
    let history_path = PathBuf::from(&config.session.history_path);
    if history_path.exists() {
        std::fs::remove_file(&history_path)?;
        println!("History cleared.");
    } else {
        println!("No history file to clear.");
    }
    Ok(())
}

fn convert_risk_config(config: &crate::config::RiskProviderConfig) -> crate::config::ProviderConfig {
    crate::config::ProviderConfig {
        r#type: config.r#type.clone(),
        model: config.model.clone(),
        base_url: config.base_url.clone(),
        api_key: config.api_key.clone(),
        temperature: config.temperature,
        max_tokens: config.max_tokens,
    }
}

async fn dry_run_risk(config: &Config, command: &str) -> Result<()> {
    let risk_model = if let Some(risk_config) = &config.risk_provider {
        build_model(&convert_risk_config(risk_config))?
    } else {
        build_model(&config.provider)?
    };

    let risk_classifier = RiskClassifier::new(risk_model);
    let cwd = std::env::current_dir()
        .map(|p| p.to_string_lossy().to_string())
        .unwrap_or_default();

    let risk = risk_classifier.evaluate(command, &cwd, &[]).await?;

    println!(
        "Risk Assessment for: {}\n  Level: {:?}\n  Reason: {}",
        command, risk.level, risk.reason
    );
    Ok(())
}