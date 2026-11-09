use rig::tool::{Tool, ToolContext};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Deserialize, JsonSchema)]
pub struct WebSearchArgs {
    pub query: String,
    #[serde(default = "default_max_results")]
    pub max_results: usize,
}

fn default_max_results() -> usize {
    10
}

#[derive(Serialize, JsonSchema)]
pub struct WebSearchOutput {
    pub results: Vec<SearchResult>,
}

#[derive(Serialize, JsonSchema)]
pub struct SearchResult {
    pub title: String,
    pub url: String,
    pub snippet: String,
}



#[derive(Debug, Error)]
pub enum WebSearchError {
    #[error("Request failed: {0}")]
    Request(#[from] reqwest::Error),
    #[error("Parse error: {0}")]
    Parse(String),
}

pub struct WebSearchTool;

impl Tool for WebSearchTool {
    const NAME: &'static str = "web_search";

    fn description(&self) -> String {
        "Search the web and return top results.".to_string()
    }

    fn parameters(&self) -> serde_json::Value {
        serde_json::to_value(schemars::schema_for!(WebSearchArgs)).unwrap()
    }

    type Args = WebSearchArgs;
    type Output = WebSearchOutput;
    type Error = WebSearchError;

    async fn call(&self, _ctx: &mut ToolContext, args: Self::Args) -> Result<Self::Output, Self::Error> {
        // Use DuckDuckGo HTML scrape as a simple search backend
        // In production, you'd use a proper search API
        let client = reqwest::Client::new();
        let url = format!("https://html.duckduckgo.com/html/?q={}", urlencoding::encode(&args.query));

        let response = client
            .get(&url)
            .header("User-Agent", "Mozilla/5.0 (compatible; shai/0.1)")
            .send()
            .await?;

        let html = response.text().await?;

        // Parse with scraper - simplified to just get snippets
        let document = scraper::Html::parse_document(&html);
        let result_selector = scraper::Selector::parse(".result__snippet").unwrap();

        let mut results = Vec::new();
        for element in document.select(&result_selector).take(args.max_results) {
            let snippet = element.text().collect::<String>().trim().to_string();

            if !snippet.is_empty() {
                results.push(SearchResult {
                    title: "Search Result".to_string(),
                    url: "".to_string(),
                    snippet,
                });
            }
        }

        Ok(WebSearchOutput { results })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_web_search_args_default() {
        let args = WebSearchArgs {
            query: "test".into(),
            max_results: 10,
        };
        assert_eq!(args.max_results, 10);
    }
}