use rig::tool::{Tool, ToolContext};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use thiserror::Error;
use tokio::sync::Mutex;

#[derive(Deserialize, JsonSchema)]
pub struct TodoReadArgs {}

#[derive(Serialize, JsonSchema)]
pub struct TodoReadOutput {
    pub todos: Vec<TodoItem>,
}

#[derive(Deserialize, JsonSchema)]
pub struct TodoWriteArgs {
    pub todos: Vec<TodoItem>,
}

#[derive(Serialize, JsonSchema)]
pub struct TodoWriteOutput {
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct TodoItem {
    pub id: String,
    pub content: String,
    pub status: TodoStatus,
    #[serde(default)]
    pub priority: Option<TodoPriority>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "lowercase")]
pub enum TodoStatus {
    Pending,
    InProgress,
    Completed,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "lowercase")]
pub enum TodoPriority {
    High,
    Medium,
    Low,
}

lazy_static::lazy_static! {
    static ref TODO_LIST: Arc<Mutex<Vec<TodoItem>>> = Arc::new(Mutex::new(Vec::new()));
}

pub struct TodoReadTool;
pub struct TodoWriteTool;

impl Tool for TodoReadTool {
    const NAME: &'static str = "todo_read";

    fn description(&self) -> String {
        "Read the current todo list.".to_string()
    }

    fn parameters(&self) -> serde_json::Value {
        serde_json::to_value(schemars::schema_for!(TodoReadArgs)).unwrap()
    }

    type Args = TodoReadArgs;
    type Output = TodoReadOutput;
    type Error = TodoError;

    async fn call(&self, _ctx: &mut ToolContext, _args: Self::Args) -> Result<Self::Output, Self::Error> {
        let list = TODO_LIST.lock().await;
        Ok(TodoReadOutput {
            todos: list.clone(),
        })
    }
}

impl Tool for TodoWriteTool {
    const NAME: &'static str = "todo_write";

    fn description(&self) -> String {
        "Write/replace the todo list.".to_string()
    }

    fn parameters(&self) -> serde_json::Value {
        serde_json::to_value(schemars::schema_for!(TodoWriteArgs)).unwrap()
    }

    type Args = TodoWriteArgs;
    type Output = TodoWriteOutput;
    type Error = TodoError;

    async fn call(&self, _ctx: &mut ToolContext, args: Self::Args) -> Result<Self::Output, Self::Error> {
        let mut list = TODO_LIST.lock().await;
        *list = args.todos;
        Ok(TodoWriteOutput { success: true })
    }
}

#[derive(Debug, Error)]
pub enum TodoError {
    #[error("Internal error")]
    Internal,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_todo_read_write() {
        // Reset the list
        {
            let mut list = TODO_LIST.lock().await;
            *list = vec![];
        }

        let mut ctx = ToolContext::default();

        // Write todos
        let write_tool = TodoWriteTool;
        let args = TodoWriteArgs {
            todos: vec![
                TodoItem {
                    id: "1".into(),
                    content: "Test task".into(),
                    status: TodoStatus::Pending,
                    priority: Some(TodoPriority::High),
                },
            ],
        };
        let result = write_tool.call(&mut ctx, args).await.unwrap();
        assert!(result.success);

        // Read todos
        let read_tool = TodoReadTool;
        let result = read_tool.call(&mut ctx, TodoReadArgs {}).await.unwrap();
        assert_eq!(result.todos.len(), 1);
        assert_eq!(result.todos[0].content, "Test task");
    }
}