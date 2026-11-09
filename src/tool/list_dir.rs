use rig::tool::{Tool, ToolContext};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;
use std::time::SystemTime;
use thiserror::Error;

#[derive(Deserialize, JsonSchema)]
pub struct ListDirArgs {
    pub path: String,
}

#[derive(Serialize, JsonSchema)]
pub struct ListDirOutput {
    pub entries: Vec<DirEntry>,
}

#[derive(Serialize, JsonSchema)]
pub struct DirEntry {
    pub name: String,
    pub is_dir: bool,
    pub size: Option<u64>,
    pub modified: Option<String>,
}



#[derive(Debug, Error)]
pub enum ListDirError {
    #[error("Not a directory: {0}")]
    NotADirectory(String),
    #[error("Permission denied: {0}")]
    PermissionDenied(String),
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}

pub struct ListDirTool;

impl Tool for ListDirTool {
    const NAME: &'static str = "list_dir";

    fn description(&self) -> String {
        "List directory contents with metadata.".to_string()
    }

    fn parameters(&self) -> serde_json::Value {
        serde_json::to_value(schemars::schema_for!(ListDirArgs)).unwrap()
    }

    type Args = ListDirArgs;
    type Output = ListDirOutput;
    type Error = ListDirError;

    async fn call(&self, _ctx: &mut ToolContext, args: Self::Args) -> Result<Self::Output, Self::Error> {
        let path = Path::new(&args.path);

        if !path.exists() {
            return Err(ListDirError::NotADirectory(format!("Path does not exist: {}", args.path)));
        }
        if !path.is_dir() {
            return Err(ListDirError::NotADirectory(format!("Not a directory: {}", args.path)));
        }

        let mut entries = Vec::new();
        for entry in fs::read_dir(path)? {
            let entry = entry?;
            let metadata = entry.metadata()?;
            let modified = metadata.modified().ok().and_then(|t| {
                t.duration_since(SystemTime::UNIX_EPOCH).ok().and_then(|duration| {
                    let secs = duration.as_secs();
                    chrono::DateTime::<chrono::Utc>::from_timestamp(secs as i64, 0)
                        .map(|dt| dt.to_rfc3339())
                })
            });

            entries.push(DirEntry {
                name: entry.file_name().to_string_lossy().to_string(),
                is_dir: metadata.is_dir(),
                size: if metadata.is_file() { Some(metadata.len()) } else { None },
                modified,
            });
        }

        // Sort: directories first, then files, both alphabetically
        entries.sort_by(|a, b| {
            match (a.is_dir, b.is_dir) {
                (true, false) => std::cmp::Ordering::Less,
                (false, true) => std::cmp::Ordering::Greater,
                _ => a.name.cmp(&b.name),
            }
        });

        Ok(ListDirOutput { entries })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;
    use std::fs;

    #[tokio::test]
    async fn test_list_dir_tool() {
        let dir = tempdir().unwrap();
        fs::write(dir.path().join("file.txt"), "hello").unwrap();
        fs::create_dir(dir.path().join("subdir")).unwrap();
        fs::write(dir.path().join("subdir").join("nested.txt"), "world").unwrap();

        let tool = ListDirTool;
        let mut ctx = ToolContext::default();
        let args = ListDirArgs {
            path: dir.path().to_string_lossy().to_string(),
        };
        let result = tool.call(&mut ctx, args).await.unwrap();
        assert_eq!(result.entries.len(), 2);
        assert!(result.entries[0].is_dir);
        assert!(!result.entries[1].is_dir);
    }
}