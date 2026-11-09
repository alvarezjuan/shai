use anyhow::Result;
use clap::{Parser, Subcommand};

mod agent;
mod config;
mod history;
mod provider;
mod repl;
mod risk;
mod tool;

#[derive(Parser)]
#[command(name = "shai", version, about = "AI shell agent")]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,

    #[arg(short, long)]
    verbose: bool,
    #[arg(short, long)]
    debug: bool,
    #[arg(short, long)]
    show_reasoning: bool,

    #[arg(num_args = 1.., value_delimiter = ' ')]
    prompt: Vec<String>,
}

#[derive(Subcommand)]
enum Commands {
    Config {
        #[command(subcommand)]
        action: ConfigAction,
    },
}

#[derive(Subcommand)]
enum ConfigAction {
    Show,
    Set { key: String, value: String },
    Init,
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    // Load config with CLI overrides
    let mut config = crate::config::Config::load()?;

    // Apply CLI flags to display config
    if cli.verbose {
        config.display.verbose = true;
    }
    if cli.debug {
        config.display.debug = true;
    }
    if cli.show_reasoning {
        config.display.show_reasoning = true;
    }

    if let Some(Commands::Config { action }) = cli.command {
        match action {
            ConfigAction::Show => {
                println!("{:#?}", config);
            }
            ConfigAction::Set { key, value } => {
                println!("Config set not yet implemented: {} = {}", key, value);
            }
            ConfigAction::Init => {
                println!("Config init not yet implemented");
            }
        }
    } else if !cli.prompt.is_empty() {
        // One-shot mode - join all prompt parts with spaces
        let prompt = cli.prompt.join(" ");
        crate::agent::run_agent(&prompt, &config).await?;
    } else {
        // REPL mode
        crate::repl::run_repl(&config).await?;
    }

    Ok(())
}
