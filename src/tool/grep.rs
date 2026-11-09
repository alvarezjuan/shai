use rig::tool::{Tool, ToolContext};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;
use thiserror::Error;
use walkdir::WalkDir;

#[derive(Deserialize, JsonSchema)]
pub struct GrepArgs {
    pub pattern: String,
    #[serde(default)]
    pub path: Option<String>,
    #[serde(default)]
    pub include: Option<String>,
}

#[derive(Serialize, JsonSchema)]
pub struct GrepOutput {
    pub matches: Vec<GrepMatch>,
}

#[derive(Serialize, JsonSchema)]
pub struct GrepMatch {
    pub file: String,
    pub line: usize,
    pub content: String,
}



#[derive(Debug, Error)]
pub enum GrepError {
    #[error("Invalid regex: {0}")]
    InvalidRegex(String),
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Walk error: {0}")]
    Walk(#[from] walkdir::Error),
}

pub struct GrepTool;

impl Tool for GrepTool {
    const NAME: &'static str = "grep";

    fn description(&self) -> String {
        "Search file contents for a regex pattern.".to_string()
    }

    fn parameters(&self) -> serde_json::Value {
        serde_json::to_value(schemars::schema_for!(GrepArgs)).unwrap()
    }

    type Args = GrepArgs;
    type Output = GrepOutput;
    type Error = GrepError;

    async fn call(&self, _ctx: &mut ToolContext, args: Self::Args) -> Result<Self::Output, Self::Error> {
        let base_path = args.path.as_deref().unwrap_or(".");
        let path = Path::new(base_path);

        if !path.exists() {
            return Err(GrepError::InvalidRegex(format!("Path does not exist: {}", base_path)));
        }

        let regex = regex::Regex::new(&args.pattern)
            .map_err(|e| GrepError::InvalidRegex(e.to_string()))?;

        let mut matches = Vec::new();
        let walker = WalkDir::new(path).follow_links(true);

        for entry in walker {
            let entry = entry?;
            if !entry.file_type().is_file() {
                continue;
            }

            // Skip if include pattern doesn't match
            if let Some(include) = &args.include {
                let file_name = entry.file_name().to_string_lossy();
                if !glob_match(include, &file_name) {
                    continue;
                }
            }

            let content = fs::read_to_string(entry.path()).unwrap_or_default();
            for (line_num, line) in content.lines().enumerate() {
                if regex.is_match(line) {
                    let rel_path = entry.path().strip_prefix(path).unwrap_or(entry.path());
                    matches.push(GrepMatch {
                        file: rel_path.to_string_lossy().to_string(),
                        line: line_num + 1,
                        content: line.to_string(),
                    });
                }
            }
        }

        Ok(GrepOutput { matches })
    }
}

fn glob_match(pattern: &str, text: &str) -> bool {
    let regex_pattern = pattern
        .replace('.', r"\.")
        .replace('*', ".*")
        .replace('?', ".");
    let regex = format!("^{}$", regex_pattern);
    regex::Regex::new(&regex).map(|re| re.is_match(text)).unwrap_or(false)
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;
    use std::fs;

    #[tokio::test]
    async fn test_grep_tool() {
        let dir = tempdir().unwrap();
        fs::write(dir.path().join("test.rs"), "fn main() {\n    println!(\"hello\");\n}").unwrap();
        fs::write(dir.path().join("test.txt"), "hello world\nfoo bar").unwrap();

        let tool = GrepTool;
        let mut ctx = ToolContext::default();
        let args = GrepArgs {
            pattern: "hello".into(),
            path: Some(dir.path().to_string_lossy().to_string()),
            include: None,
        };
        let result = tool.call(&mut ctx, args).await.unwrap();
        assert_eq!(result.matches.len(), 2);
    }

    #[tokio::test]
    async fn test_grep_with_include() {
        let dir = tempdir().unwrap();
        fs::write(dir.path().join("test.rs"), "fn main() {}").unwrap();
        fs::write(dir.path().join("test.txt"), "fn main() {}").unwrap();

        let tool = GrepTool;
        let mut ctx = ToolContext::default();
        let args = GrepArgs {
            pattern: "fn main".into(),
            path: Some(dir.path().to_string_lossy().to_string()),
            include: Some("*.rs".into()),
        };
        let result = tool.call(&mut ctx, args).await.unwrap();
        assert_eq!(result.matches.len(), 1);
        assert!(result.matches[0].file.ends_with(".rs"));
    }
}