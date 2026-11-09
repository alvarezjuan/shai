use anyhow::Result;
use dirs;
use serde::{Deserialize, Serialize};
use std::env;
use std::fs;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub provider: ProviderConfig,
    pub risk_provider: Option<RiskProviderConfig>,
    pub session: SessionConfig,
    pub display: DisplayConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProviderConfig {
    pub r#type: String,
    pub model: String,
    pub base_url: Option<String>,
    pub api_key: Option<String>,
    pub temperature: f32,
    pub max_tokens: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiskProviderConfig {
    pub r#type: String,
    pub model: String,
    pub base_url: Option<String>,
    pub api_key: Option<String>,
    pub temperature: f32,
    pub max_tokens: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionConfig {
    pub history_path: String,
    pub max_history_turns: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DisplayConfig {
    pub verbose: bool,
    pub debug: bool,
    pub show_reasoning: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            provider: ProviderConfig {
                r#type: "openai".into(),
                model: "gpt-4o".into(),
                base_url: None,
                api_key: None,
                temperature: 0.2,
                max_tokens: 4096,
            },
            risk_provider: None,
            session: SessionConfig {
                history_path: ".shai/session.jsonl".into(),
                max_history_turns: 50,
            },
            display: DisplayConfig {
                verbose: false,
                debug: false,
                show_reasoning: false,
            },
        }
    }
}

impl Config {
    pub fn load() -> Result<Self> {
        let mut config = Config::default();

        // 1. Load global config: ~/.config/shai/config.toml
        if let Some(config_dir) = dirs::config_dir() {
            let global_path = config_dir.join("shai").join("config.toml");
            if global_path.exists() {
                let content = fs::read_to_string(&global_path)?;
                if let Ok(toml_config) = toml::from_str::<Config>(&content) {
                    config = merge_config(config, toml_config);
                }
            }
        }

        // 2. Load project config: .shai/config.toml
        if let Ok(cwd) = env::current_dir() {
            let project_path = cwd.join(".shai").join("config.toml");
            if project_path.exists() {
                let content = fs::read_to_string(&project_path)?;
                if let Ok(toml_config) = toml::from_str::<Config>(&content) {
                    config = merge_config(config, toml_config);
                }
            }
        }

        // 3. Override with environment variables
        config = apply_env_overrides(config)?;

        // 4. If risk_provider is not set, default to provider with risk-appropriate settings
        if config.risk_provider.is_none() {
            config.risk_provider = Some(RiskProviderConfig {
                r#type: config.provider.r#type.clone(),
                model: match config.provider.r#type.as_str() {
                    "openai" => "gpt-4o-mini".into(),
                    "anthropic" => "claude-3-haiku".into(),
                    "ollama" => "llama3.1".into(),
                    _ => "gpt-4o-mini".into(),
                },
                base_url: config.provider.base_url.clone(),
                api_key: config.provider.api_key.clone(),
                temperature: 0.0,
                max_tokens: 100,
            });
        }

        Ok(config)
    }
}

fn merge_config(base: Config, overlay: Config) -> Config {
    Config {
        provider: merge_provider(base.provider, overlay.provider),
        risk_provider: merge_risk_provider(base.risk_provider, overlay.risk_provider),
        session: merge_session(base.session, overlay.session),
        display: merge_display(base.display, overlay.display),
    }
}

fn merge_provider(base: ProviderConfig, overlay: ProviderConfig) -> ProviderConfig {
    ProviderConfig {
        r#type: if overlay.r#type.is_empty() { base.r#type } else { overlay.r#type },
        model: if overlay.model.is_empty() { base.model } else { overlay.model },
        base_url: overlay.base_url.or(base.base_url),
        api_key: overlay.api_key.or(base.api_key),
        temperature: if overlay.temperature == 0.0 { base.temperature } else { overlay.temperature },
        max_tokens: if overlay.max_tokens == 0 { base.max_tokens } else { overlay.max_tokens },
    }
}

fn merge_risk_provider(base: Option<RiskProviderConfig>, overlay: Option<RiskProviderConfig>) -> Option<RiskProviderConfig> {
    match (base, overlay) {
        (Some(b), Some(o)) => Some(RiskProviderConfig {
            r#type: if o.r#type.is_empty() { b.r#type } else { o.r#type },
            model: if o.model.is_empty() { b.model } else { o.model },
            base_url: o.base_url.or(b.base_url),
            api_key: o.api_key.or(b.api_key),
            temperature: if o.temperature == 0.0 { b.temperature } else { o.temperature },
            max_tokens: if o.max_tokens == 0 { b.max_tokens } else { o.max_tokens },
        }),
        (Some(b), None) => Some(b),
        (None, Some(o)) => Some(o),
        (None, None) => None,
    }
}

fn merge_session(base: SessionConfig, overlay: SessionConfig) -> SessionConfig {
    SessionConfig {
        history_path: if overlay.history_path.is_empty() { base.history_path } else { overlay.history_path },
        max_history_turns: if overlay.max_history_turns == 0 { base.max_history_turns } else { overlay.max_history_turns },
    }
}

fn merge_display(base: DisplayConfig, overlay: DisplayConfig) -> DisplayConfig {
    DisplayConfig {
        verbose: overlay.verbose || base.verbose,
        debug: overlay.debug || base.debug,
        show_reasoning: overlay.show_reasoning || base.show_reasoning,
    }
}

fn apply_env_overrides(mut config: Config) -> Result<Config> {
    // Primary provider env vars
    if let Ok(v) = env::var("SHAI_PROVIDER") { config.provider.r#type = v; }
    if let Ok(v) = env::var("SHAI_MODEL") { config.provider.model = v; }
    if let Ok(v) = env::var("SHAI_BASE_URL") { config.provider.base_url = Some(v); }
    if let Ok(v) = env::var("SHAI_API_KEY") { config.provider.api_key = Some(v); }
    if let Ok(v) = env::var("SHAI_TEMPERATURE") { config.provider.temperature = v.parse()?; }
    if let Ok(v) = env::var("SHAI_MAX_TOKENS") { config.provider.max_tokens = v.parse()?; }

    // Provider-specific fallbacks
    if config.provider.api_key.is_none() {
        if let Ok(v) = env::var("OPENAI_API_KEY") { config.provider.api_key = Some(v); }
        else if let Ok(v) = env::var("ANTHROPIC_API_KEY") { config.provider.api_key = Some(v); }
    }
    if config.provider.base_url.is_none() {
        if let Ok(v) = env::var("OLLAMA_API_BASE_URL") { config.provider.base_url = Some(v); }
    }

    // Risk provider env vars
    if let Some(ref mut risk) = config.risk_provider {
        if let Ok(v) = env::var("SHAI_RISK_PROVIDER") { risk.r#type = v; }
        if let Ok(v) = env::var("SHAI_RISK_MODEL") { risk.model = v; }
        if let Ok(v) = env::var("SHAI_RISK_BASE_URL") { risk.base_url = Some(v); }
        if let Ok(v) = env::var("SHAI_RISK_API_KEY") { risk.api_key = Some(v); }
        if let Ok(v) = env::var("SHAI_RISK_TEMPERATURE") { risk.temperature = v.parse()?; }
        if let Ok(v) = env::var("SHAI_RISK_MAX_TOKENS") { risk.max_tokens = v.parse()?; }
    }

    // Display flags
    if let Ok(v) = env::var("SHAI_VERBOSE") { config.display.verbose = v.parse()?; }
    if let Ok(v) = env::var("SHAI_DEBUG") { config.display.debug = v.parse()?; }
    if let Ok(v) = env::var("SHAI_SHOW_REASONING") { config.display.show_reasoning = v.parse()?; }

    Ok(config)
}