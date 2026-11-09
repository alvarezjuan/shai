use crate::config::ProviderConfig;
use anyhow::Result;
use rig::driver::DynModel;
use rig::operation::Completion;
use rig::providers::anthropic::AnthropicConfig;
use rig::providers::ollama::Ollama;
use rig::providers::openai::OpenAIConfig;

pub fn build_model(config: &ProviderConfig) -> Result<DynModel<Completion>> {
    match config.r#type.as_str() {
        "openai" => {
            let api_key = config.api_key.clone().unwrap_or_default();
            let mut client_config = OpenAIConfig::new(api_key);
            if let Some(url) = &config.base_url {
                client_config = client_config.with_base_url(url);
            }
            let client = client_config.client();
            Ok(client.chat(&config.model).into())
        }
        "anthropic" => {
            let api_key = config.api_key.clone().unwrap_or_default();
            let mut client_config = AnthropicConfig::new(api_key);
            if let Some(url) = &config.base_url {
                client_config = client_config.with_base_url(url);
            }
            let client = client_config.client();
            Ok(client.completion(&config.model).into())
        }
        "ollama" => {
            let client = Ollama::new();
            // Base URL can be set via OLLAMA_API_BASE_URL env var
            Ok(client.completion(&config.model).into())
        }
        _ => anyhow::bail!("Unknown provider type: {}", config.r#type),
    }
}
