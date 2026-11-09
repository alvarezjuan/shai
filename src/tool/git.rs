use rig::tool::{Tool, ToolContext};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Deserialize, JsonSchema)]
pub struct GitStatusArgs {}

#[derive(Serialize, JsonSchema)]
pub struct GitStatusOutput {
    pub status: String,
}

#[derive(Deserialize, JsonSchema)]
pub struct GitDiffArgs {
    #[serde(default)]
    pub staged: bool,
}

#[derive(Serialize, JsonSchema)]
pub struct GitDiffOutput {
    pub diff: String,
}

#[derive(Deserialize, JsonSchema)]
pub struct GitLogArgs {
    #[serde(default = "default_log_limit")]
    pub limit: usize,
}

fn default_log_limit() -> usize {
    10
}

#[derive(Serialize, JsonSchema)]
pub struct GitLogOutput {
    pub log: String,
}

#[derive(Deserialize, JsonSchema)]
pub struct GitCommitArgs {
    pub message: String,
    #[serde(default)]
    pub add_all: bool,
}

#[derive(Serialize, JsonSchema)]
pub struct GitCommitOutput {
    pub commit_hash: String,
}

#[derive(Deserialize, JsonSchema, Debug)]
pub struct GitPushArgs {
    #[serde(default)]
    pub remote: Option<String>,
    #[serde(default)]
    pub branch: Option<String>,
}

#[derive(Serialize, JsonSchema)]
pub struct GitPushOutput {
    pub output: String,
}

#[derive(Debug, Error)]
pub enum GitError {
    #[error("Not a git repository")]
    NotARepo,
    #[error("Git error: {0}")]
    Other(String),
}

pub struct GitStatusTool;
pub struct GitDiffTool;
pub struct GitLogTool;
pub struct GitCommitTool;
pub struct GitPushTool;

impl Tool for GitStatusTool {
    const NAME: &'static str = "git_status";

    fn description(&self) -> String {
        "Show git status.".to_string()
    }

    fn parameters(&self) -> serde_json::Value {
        serde_json::to_value(schemars::schema_for!(GitStatusArgs)).unwrap()
    }

    type Args = GitStatusArgs;
    type Output = GitStatusOutput;
    type Error = GitError;

    async fn call(&self, _ctx: &mut ToolContext, _args: Self::Args) -> Result<Self::Output, Self::Error> {
        // TODO: Implement with git2
        Ok(GitStatusOutput {
            status: "Git status not yet implemented (needs git2 with openssl)".into(),
        })
    }
}

impl Tool for GitDiffTool {
    const NAME: &'static str = "git_diff";

    fn description(&self) -> String {
        "Show git diff.".to_string()
    }

    fn parameters(&self) -> serde_json::Value {
        serde_json::to_value(schemars::schema_for!(GitDiffArgs)).unwrap()
    }

    type Args = GitDiffArgs;
    type Output = GitDiffOutput;
    type Error = GitError;

    async fn call(&self, _ctx: &mut ToolContext, _args: Self::Args) -> Result<Self::Output, Self::Error> {
        // TODO: Implement with git2
        Ok(GitDiffOutput {
            diff: "Git diff not yet implemented (needs git2 with openssl)".into(),
        })
    }
}

impl Tool for GitLogTool {
    const NAME: &'static str = "git_log";

    fn description(&self) -> String {
        "Show git log.".to_string()
    }

    fn parameters(&self) -> serde_json::Value {
        serde_json::to_value(schemars::schema_for!(GitLogArgs)).unwrap()
    }

    type Args = GitLogArgs;
    type Output = GitLogOutput;
    type Error = GitError;

    async fn call(&self, _ctx: &mut ToolContext, _args: Self::Args) -> Result<Self::Output, Self::Error> {
        // TODO: Implement with git2
        Ok(GitLogOutput {
            log: "Git log not yet implemented (needs git2 with openssl)".into(),
        })
    }
}

impl Tool for GitCommitTool {
    const NAME: &'static str = "git_commit";

    fn description(&self) -> String {
        "Commit staged changes.".to_string()
    }

    fn parameters(&self) -> serde_json::Value {
        serde_json::to_value(schemars::schema_for!(GitCommitArgs)).unwrap()
    }

    type Args = GitCommitArgs;
    type Output = GitCommitOutput;
    type Error = GitError;

    async fn call(&self, _ctx: &mut ToolContext, _args: Self::Args) -> Result<Self::Output, Self::Error> {
        // TODO: Implement with git2
        Ok(GitCommitOutput {
            commit_hash: "Git commit not yet implemented (needs git2 with openssl)".into(),
        })
    }
}

impl Tool for GitPushTool {
    const NAME: &'static str = "git_push";

    fn description(&self) -> String {
        "Push to remote.".to_string()
    }

    fn parameters(&self) -> serde_json::Value {
        serde_json::to_value(schemars::schema_for!(GitPushArgs)).unwrap()
    }

    type Args = GitPushArgs;
    type Output = GitPushOutput;
    type Error = GitError;

    async fn call(&self, _ctx: &mut ToolContext, _args: Self::Args) -> Result<Self::Output, Self::Error> {
        // TODO: Implement with git2
        Ok(GitPushOutput {
            output: "Git push not yet implemented (needs git2 with openssl)".into(),
        })
    }
}