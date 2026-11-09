use rig::tool::{Tool, ToolContext};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Deserialize, JsonSchema)]
pub struct WebFetchArgs {
    pub url: String,
    #[serde(default = "default_timeout")]
    pub timeout_secs: u64,
}

fn default_timeout() -> u64 {
    30
}

#[derive(Serialize, JsonSchema)]
pub struct WebFetchOutput {
    pub url: String,
    pub title: String,
    pub content: String,
}



#[derive(Debug, Error)]
pub enum WebFetchError {
    #[error("Request failed: {0}")]
    Request(#[from] reqwest::Error),
    #[error("Parse error: {0}")]
    Parse(String),
    #[error("Invalid URL: {0}")]
    InvalidUrl(#[from] url::ParseError),
}

pub struct WebFetchTool;

impl Tool for WebFetchTool {
    const NAME: &'static str = "web_fetch";

    fn description(&self) -> String {
        "Fetch and extract text content from a URL.".to_string()
    }

    fn parameters(&self) -> serde_json::Value {
        serde_json::to_value(schemars::schema_for!(WebFetchArgs)).unwrap()
    }

    type Args = WebFetchArgs;
    type Output = WebFetchOutput;
    type Error = WebFetchError;

    async fn call(&self, _ctx: &mut ToolContext, args: Self::Args) -> Result<Self::Output, Self::Error> {
        let parsed_url = url::Url::parse(&args.url)?;

        let client = reqwest::Client::new();
        let response = client
            .get(parsed_url.clone())
            .header("User-Agent", "Mozilla/5.0 (compatible; shai/0.1)")
            .timeout(std::time::Duration::from_secs(args.timeout_secs))
            .send()
            .await?;

        let html = response.text().await?;

        // Parse with scraper
        let document = scraper::Html::parse_document(&html);

        // Extract title
        let title_selector = scraper::Selector::parse("title").unwrap();
        let title = document
            .select(&title_selector)
            .next()
            .map(|e| e.text().collect::<String>().trim().to_string())
            .unwrap_or_default();

        // Extract main content - simplified approach
        let body_selector = scraper::Selector::parse("body").unwrap();
        let content = if let Some(body) = document.select(&body_selector).next() {
            // Just get all text from body
            body.text().collect::<String>().trim().to_string()
        } else {
            // Fallback: get all text
            document.root_element().text().collect::<String>().trim().to_string()
        };

        Ok(WebFetchOutput {
            url: args.url,
            title,
            content,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_web_fetch_args_default() {
        let args = WebFetchArgs {
            url: "https://example.com".into(),
            timeout_secs: 30,
        };
        assert_eq!(args.timeout_secs, 30);
    }
}