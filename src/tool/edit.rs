use rig::tool::{Tool, ToolContext};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;
use thiserror::Error;

#[derive(Deserialize, JsonSchema)]
pub struct EditArgs {
    pub path: String,
    pub old_string: String,
    pub new_string: String,
}

#[derive(Serialize, JsonSchema)]
pub struct EditOutput {
    pub diff: String,
}

#[derive(Deserialize, JsonSchema)]
pub struct PatchArgs {
    pub path: String,
    pub patch: String,
}

#[derive(Serialize, JsonSchema)]
pub struct PatchOutput {
    pub applied: bool,
}

#[derive(Debug, Error)]
pub enum EditError {
    #[error("File not found: {0}")]
    NotFound(String),
    #[error("Old string not found in file")]
    OldStringNotFound,
    #[error("Multiple matches for old string")]
    MultipleMatches,
    #[error("Invalid path: {0}")]
    InvalidPath(String),
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Patch error: {0}")]
    Patch(String),
}

pub struct EditTool;
pub struct PatchTool;

impl Tool for EditTool {
    const NAME: &'static str = "edit";

    fn description(&self) -> String {
        "Edit a file by replacing a string with another string.".to_string()
    }

    fn parameters(&self) -> serde_json::Value {
        serde_json::to_value(schemars::schema_for!(EditArgs)).unwrap()
    }

    type Args = EditArgs;
    type Output = EditOutput;
    type Error = EditError;

    async fn call(&self, _ctx: &mut ToolContext, args: Self::Args) -> Result<Self::Output, Self::Error> {
        let path = Path::new(&args.path);

        // Validate path is within current working directory
        let cwd = std::env::current_dir().map_err(|e| EditError::Io(e))?;
        let canonical_cwd = cwd.canonicalize().map_err(|e| EditError::Io(e))?;
        let canonical_path = path.canonicalize().map_err(|e| EditError::Io(e))?;

        if !canonical_path.starts_with(&canonical_cwd) {
            return Err(EditError::InvalidPath(
                "Path must be within current working directory".into(),
            ));
        }

        let content = fs::read_to_string(&canonical_path)?;

        let count = content.matches(&args.old_string).count();
        if count == 0 {
            return Err(EditError::OldStringNotFound);
        }
        if count > 1 {
            return Err(EditError::MultipleMatches);
        }

        let new_content = content.replace(&args.old_string, &args.new_string);
        fs::write(&canonical_path, &new_content)?;

        // Generate a simple diff
        let diff = generate_diff(&content, &new_content, &args.path);

        Ok(EditOutput { diff })
    }
}

impl Tool for PatchTool {
    const NAME: &'static str = "patch";

    fn description(&self) -> String {
        "Apply a unified diff patch to a file.".to_string()
    }

    fn parameters(&self) -> serde_json::Value {
        serde_json::to_value(schemars::schema_for!(PatchArgs)).unwrap()
    }

    type Args = PatchArgs;
    type Output = PatchOutput;
    type Error = EditError;

    async fn call(&self, _ctx: &mut ToolContext, args: Self::Args) -> Result<Self::Output, Self::Error> {
        let path = Path::new(&args.path);

        // Validate path is within current working directory
        let cwd = std::env::current_dir().map_err(|e| EditError::Io(e))?;
        let canonical_cwd = cwd.canonicalize().map_err(|e| EditError::Io(e))?;
        let canonical_path = path.canonicalize().map_err(|e| EditError::Io(e))?;

        if !canonical_path.starts_with(&canonical_cwd) {
            return Err(EditError::InvalidPath(
                "Path must be within current working directory".into(),
            ));
        }

        let content = fs::read_to_string(&canonical_path)?;
        let new_content = apply_patch(&content, &args.patch)?;

        fs::write(&canonical_path, &new_content)?;

        Ok(PatchOutput { applied: true })
    }
}

fn generate_diff(old: &str, new: &str, path: &str) -> String {
    let mut output = String::new();
    output.push_str(&format!("--- a/{}\n", path));
    output.push_str(&format!("+++ b/{}\n", path));
    
    let old_lines: Vec<&str> = old.lines().collect();
    let new_lines: Vec<&str> = new.lines().collect();
    
    // Simple diff - just show all old lines as removed and new lines as added
    // A proper diff would use LCS algorithm
    for line in &old_lines {
        output.push_str(&format!("-{}\n", line));
    }
    for line in &new_lines {
        output.push_str(&format!("+{}\n", line));
    }
    output
}

fn apply_patch(content: &str, patch: &str) -> Result<String, EditError> {
    let lines: Vec<&str> = content.lines().collect();
    let mut result = Vec::new();
    let mut i = 0;

    for line in patch.lines() {
        if line.starts_with("---") || line.starts_with("+++") || line.starts_with("@@") {
            continue;
        }
        if line.starts_with(' ') {
            // Context line
            if i < lines.len() {
                result.push(lines[i].to_string());
                i += 1;
            }
        } else if line.starts_with('-') {
            // Deleted line - skip in old content
            if i < lines.len() && lines[i] == &line[1..] {
                i += 1;
            }
        } else if line.starts_with('+') {
            // Added line
            result.push(line[1..].to_string());
        }
    }

    // Add remaining lines
    while i < lines.len() {
        result.push(lines[i].to_string());
        i += 1;
    }

    Ok(result.join("\n"))
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::Builder;
    use std::fs;
    use std::env;

    #[tokio::test]
    async fn test_edit_tool() {
        let cwd = env::current_dir().unwrap();
        let dir = Builder::new().prefix("shai_test_").tempdir_in(&cwd).unwrap();
        let file = dir.path().join("test.txt");
        fs::write(&file, "hello world").unwrap();

        let tool = EditTool;
        let mut ctx = ToolContext::default();
        let args = EditArgs {
            path: file.to_string_lossy().to_string(),
            old_string: "world".into(),
            new_string: "rust".into(),
        };
        let result = tool.call(&mut ctx, args).await.unwrap();
        let content = fs::read_to_string(&file).unwrap();
        assert_eq!(content, "hello rust");
        // The diff shows all old lines with - and new lines with +
        assert!(result.diff.contains("-hello world"));
        assert!(result.diff.contains("+hello rust"));
    }

    #[tokio::test]
    async fn test_edit_tool_not_found() {
        let cwd = env::current_dir().unwrap();
        let dir = Builder::new().prefix("shai_test_").tempdir_in(&cwd).unwrap();
        let file = dir.path().join("test.txt");
        fs::write(&file, "hello world").unwrap();

        let tool = EditTool;
        let mut ctx = ToolContext::default();
        let args = EditArgs {
            path: file.to_string_lossy().to_string(),
            old_string: "foo".into(),
            new_string: "bar".into(),
        };
        let result = tool.call(&mut ctx, args).await;
        assert!(result.is_err());
    }
}