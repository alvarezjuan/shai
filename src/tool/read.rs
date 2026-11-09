use rig::tool::{Tool, ToolContext};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;
use thiserror::Error;

#[derive(Deserialize, JsonSchema)]
pub struct ReadArgs {
    pub path: String,
    #[serde(default)]
    pub offset: Option<usize>,
    #[serde(default)]
    pub limit: Option<usize>,
}

#[derive(Serialize, JsonSchema)]
pub struct ReadOutput {
    pub content: String,
}



#[derive(Debug, Error)]
pub enum ReadError {
    #[error("File not found: {0}")]
    NotFound(String),
    #[error("Permission denied: {0}")]
    PermissionDenied(String),
    #[error("Invalid path: {0}")]
    InvalidPath(String),
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}

pub struct ReadTool;

impl Tool for ReadTool {
    const NAME: &'static str = "read";

    fn description(&self) -> String {
        "Read file contents with optional offset/limit.".to_string()
    }

    fn parameters(&self) -> serde_json::Value {
        serde_json::to_value(schemars::schema_for!(ReadArgs)).unwrap()
    }

    type Args = ReadArgs;
    type Output = ReadOutput;
    type Error = ReadError;

    async fn call(&self, _ctx: &mut ToolContext, args: Self::Args) -> Result<Self::Output, Self::Error> {
        let path = Path::new(&args.path);

        // Validate path is within current working directory (no directory traversal)
        let cwd = std::env::current_dir().map_err(|e| ReadError::Io(e))?;
        let canonical_cwd = cwd.canonicalize().map_err(|e| ReadError::Io(e))?;
        let canonical_path = path.canonicalize().map_err(|e| ReadError::Io(e))?;

        if !canonical_path.starts_with(&canonical_cwd) {
            return Err(ReadError::InvalidPath(
                "Path must be within current working directory".into(),
            ));
        }

        let content = fs::read_to_string(&canonical_path)?;

        let start = args.offset.unwrap_or(0);
        let end = match args.limit {
            Some(limit) => (start + limit).min(content.len()),
            None => content.len(),
        };

        let content = if start < content.len() {
            content[start..end].to_string()
        } else {
            String::new()
        };

        Ok(ReadOutput { content })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::Builder;
    use std::env;

    #[tokio::test]
    async fn test_read_tool() {
        let cwd = env::current_dir().unwrap();
        let dir = Builder::new().prefix("shai_test_").tempdir_in(&cwd).unwrap();
        let file = dir.path().join("test.txt");
        fs::write(&file, "hello world").unwrap();

        let tool = ReadTool;
        let mut ctx = ToolContext::default();
        let args = ReadArgs {
            path: file.to_string_lossy().to_string(),
            offset: None,
            limit: None,
        };
        let result = tool.call(&mut ctx, args).await.unwrap();
        assert_eq!(result.content, "hello world");
    }

    #[tokio::test]
    async fn test_read_tool_with_offset_limit() {
        let cwd = env::current_dir().unwrap();
        let dir = Builder::new().prefix("shai_test_").tempdir_in(&cwd).unwrap();
        let file = dir.path().join("test.txt");
        fs::write(&file, "hello world").unwrap();

        let tool = ReadTool;
        let mut ctx = ToolContext::default();
        let args = ReadArgs {
            path: file.to_string_lossy().to_string(),
            offset: Some(6),
            limit: Some(5),
        };
        let result = tool.call(&mut ctx, args).await.unwrap();
        assert_eq!(result.content, "world");
    }

    #[tokio::test]
    async fn test_read_tool_nonexistent() {
        let tool = ReadTool;
        let mut ctx = ToolContext::default();
        let args = ReadArgs {
            path: "/nonexistent/file.txt".into(),
            offset: None,
            limit: None,
        };
        let result = tool.call(&mut ctx, args).await;
        assert!(result.is_err());
    }
}