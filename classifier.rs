use crate::risk::prompt::{build_risk_prompt, RiskResult};
use anyhow::Result;
use rig::completion::CompletionRequest;
use rig::driver::DynModel;
use rig::operation::Completion;

use std::sync::Arc;

#[derive(Clone)]
pub struct RiskClassifier {
    model: Arc<DynModel<Completion>>,
}

impl RiskClassifier {
    pub fn new(model: DynModel<Completion>) -> Self {
        Self {
            model: Arc::new(model),
        }
    }

    pub async fn evaluate(
        &self,
        command: &str,
        cwd: &str,
        recent_commands: &[String],
    ) -> Result<RiskResult> {
        let prompt = build_risk_prompt(command, cwd, recent_commands);
        let request = CompletionRequest::new(prompt);
        let response = self.model.call(request).await?;
        let text = response.text();

        let json = extract_json(&text)?;
        let result: RiskResult = serde_json::from_str(&json)?;
        Ok(result)
    }
}

fn extract_json(text: &str) -> Result<String> {
    if let Some(start) = text.find('{') {
        if let Some(end) = text.rfind('}') {
            return Ok(text[start..=end].to_string());
        }
    }
    anyhow::bail!("No JSON found in response: {}", text)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_json_simple() {
        let text = r#"{"level": 1, "reason": "test"}"#;
        let result = extract_json(text).unwrap();
        assert_eq!(result, r#"{"level": 1, "reason": "test"}"#);
    }

    #[test]
    fn test_extract_json_with_markdown() {
        let text = r#"```json
{"level": 2, "reason": "reads sensitive file"}
```"#;
        let result = extract_json(text).unwrap();
        assert_eq!(result, r#"{"level": 2, "reason": "reads sensitive file"}"#);
    }

    #[test]
    fn test_extract_json_with_extra_text() {
        let text = r#"The classification is: {"level": 3, "reason": "writes file"} and that's it."#;
        let result = extract_json(text).unwrap();
        assert_eq!(result, r#"{"level": 3, "reason": "writes file"}"#);
    }
}
