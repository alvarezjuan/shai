use rig::tool::{Tool, ToolContext};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::path::Path;
use thiserror::Error;
use walkdir::WalkDir;

#[derive(Deserialize, JsonSchema)]
pub struct GlobArgs {
    pub pattern: String,
    #[serde(default)]
    pub path: Option<String>,
}

#[derive(Serialize, JsonSchema)]
pub struct GlobOutput {
    pub matches: Vec<String>,
}



#[derive(Debug, Error)]
pub enum GlobError {
    #[error("Invalid pattern: {0}")]
    InvalidPattern(String),
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Walk error: {0}")]
    Walk(#[from] walkdir::Error),
}

pub struct GlobTool;

impl Tool for GlobTool {
    const NAME: &'static str = "glob";

    fn description(&self) -> String {
        "Find files matching a glob pattern.".to_string()
    }

    fn parameters(&self) -> serde_json::Value {
        serde_json::to_value(schemars::schema_for!(GlobArgs)).unwrap()
    }

    type Args = GlobArgs;
    type Output = GlobOutput;
    type Error = GlobError;

    async fn call(&self, _ctx: &mut ToolContext, args: Self::Args) -> Result<Self::Output, Self::Error> {
        let base_path = args.path.as_deref().unwrap_or(".");
        let path = Path::new(base_path);

        if !path.exists() {
            return Err(GlobError::InvalidPattern(format!("Path does not exist: {}", base_path)));
        }

        let mut matches = Vec::new();
        let walker = WalkDir::new(path).follow_links(true);

        for entry in walker {
            let entry = entry?;
            let rel_path = entry.path().strip_prefix(path).unwrap_or(entry.path());
            let rel_str = rel_path.to_string_lossy();

            if glob_match(&args.pattern, &rel_str) {
                matches.push(rel_str.to_string());
            }
        }

        matches.sort();
        Ok(GlobOutput { matches })
    }
}

fn glob_match(pattern: &str, text: &str) -> bool {
    // Simple glob matching: * matches anything, ? matches single char
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
    async fn test_glob_tool() {
        let dir = tempdir().unwrap();
        fs::write(dir.path().join("test.rs"), "fn main() {}").unwrap();
        fs::write(dir.path().join("test.txt"), "hello").unwrap();
        fs::create_dir(dir.path().join("subdir")).unwrap();
        fs::write(dir.path().join("subdir").join("nested.rs"), "mod test;").unwrap();

        let tool = GlobTool;
        let mut ctx = ToolContext::default();
        let args = GlobArgs {
            pattern: "*.rs".into(),
            path: Some(dir.path().to_string_lossy().to_string()),
        };
        let result = tool.call(&mut ctx, args).await.unwrap();
        assert_eq!(result.matches.len(), 2);
        assert!(result.matches.contains(&"test.rs".to_string()));
        assert!(result.matches.contains(&"subdir/nested.rs".to_string()));
    }

    #[tokio::test]
    async fn test_glob_recursive() {
        let dir = tempdir().unwrap();
        fs::create_dir_all(dir.path().join("a").join("b")).unwrap();
        fs::write(dir.path().join("a").join("b").join("deep.rs"), "").unwrap();

        let tool = GlobTool;
        let mut ctx = ToolContext::default();
        let args = GlobArgs {
            pattern: "**/*.rs".into(),
            path: Some(dir.path().to_string_lossy().to_string()),
        };
        let result = tool.call(&mut ctx, args).await.unwrap();
        assert_eq!(result.matches.len(), 1);
        assert!(result.matches.contains(&"a/b/deep.rs".to_string()));
    }
}