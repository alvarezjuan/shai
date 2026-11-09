# Test Templates

Reusable test patterns and templates for the shai codebase.

---

## Basic Unit Test

```rust
#[tokio::test]
async fn test_my_function_valid_input_returns_expected() {
    // Arrange
    let input = create_valid_input();
    
    // Act
    let result = my_function(input).await;
    
    // Assert
    assert!(result.is_ok());
    assert_eq!(result.unwrap().field, "expected");
}
```

---

## Error Case Test

```rust
#[tokio::test]
async fn test_my_function_invalid_input_returns_error() {
    let input = create_invalid_input();
    let result = my_function(input).await;
    assert!(result.is_err());
    assert!(matches!(result.unwrap_err(), MyError::ExpectedVariant));
}
```

---

## Filesystem Test (Critical: CWD Requirement)

```rust
#[tokio::test]
async fn test_file_tool_reads_content() {
    // CRITICAL: Create temp dir within CWD for path validation
    let cwd = env::current_dir().unwrap();
    let dir = Builder::new().prefix("shai_test_").tempdir_in(&cwd).unwrap();
    let file = dir.path().join("test.txt");
    fs::write(&file, "hello").unwrap();

    let tool = MyFileTool;
    let mut ctx = ToolContext::default();
    let args = MyArgs { path: file.to_string_lossy().to_string() };
    let result = tool.call(&mut ctx, args).await.unwrap();
    
    assert_eq!(result.content, "hello");
}
```

---

## Parametrized Test (using `rstest`)

```rust
#[rstest]
#[case(RiskLevel::Allowed, "ls")]
#[case(RiskLevel::ReadSensitive, "cat /etc/passwd")]
#[case(RiskLevel::WriteRecoverable, "rm file.txt")]
#[case(RiskLevel::Catastrophic, "rm -rf /")]
fn test_risk_classification(#[case] expected: RiskLevel, #[case] command: &str) {
    let result = classify(command).await.unwrap();
    assert_eq!(result.level, expected);
}
```

---

## Async Test with Multiple Assertions

```rust
#[tokio::test]
async fn test_shell_tool_execute_command() {
    let output = execute_command("echo hello").await.unwrap();
    
    // Multiple assertions for different aspects
    assert_eq!(output.exit_code, 0);
    assert_eq!(output.stdout.trim(), "hello");
    assert!(output.stderr.is_empty());
}
```

---

## Testing Error Variants

```rust
#[tokio::test]
async fn test_read_tool_path_not_found() {
    let tool = ReadTool;
    let mut ctx = ToolContext::default();
    let args = ReadArgs { 
        path: "/nonexistent/file.txt".into(), 
        offset: None, 
        limit: None 
    };
    let result = tool.call(&mut ctx, args).await;
    
    assert!(result.is_err());
    match result.unwrap_err() {
        ReadError::NotFound(_) => {}, // Expected
        e => panic!("Expected NotFound error, got: {:?}", e),
    }
}
```

---

## Testing Multiple Scenarios (Table-Driven)

```rust
#[test]
fn test_glob_pattern_matching() {
    struct Case {
        pattern: &'static str,
        input: &'static str,
        expected: bool,
    }
    
    let cases = vec![
        Case { pattern: "*.rs", input: "test.rs", expected: true },
        Case { pattern: "*.rs", input: "test.txt", expected: false },
        Case { pattern: "test.*", input: "test.rs", expected: true },
        Case { pattern: "test.*", input: "other.rs", expected: false },
        Case { pattern: "**/*.rs", input: "dir/test.rs", expected: true },
    ];
    
    for case in cases {
        let result = glob_match(case.pattern, case.input);
        assert_eq!(result, case.expected, "Failed for pattern: {}, input: {}", case.pattern, case.input);
    }
}
```

---

## Testing Async Functions with Timeouts

```rust
#[tokio::test]
async fn test_shell_tool_timeout() {
    // Command that should timeout
    let result = timeout(Duration::from_secs(1), execute_command("sleep 10")).await;
    
    assert!(result.is_err());
    assert!(matches!(result.unwrap_err(), ShellError::Timeout(_)));
}
```

---

## Testing Tool Dispatch

```rust
#[tokio::test]
async fn test_tool_registry_dispatch_shell() {
    let tool_call = ToolCall {
        id: "test-1".into(),
        name: "shell".into(),
        function: ToolFunction {
            name: "shell".into(),
            arguments: serde_json::json!({ "command": "echo test" }),
        },
    };
    
    let risk_classifier = RiskClassifier::new(mock_risk_model());
    let config = Config::default();
    
    let result = ToolRegistry::dispatch(&tool_call, &risk_classifier, &config).await;
    
    assert!(result.is_ok());
    assert!(result.unwrap().is_success());
}
```

---

## Testing with Mock Models

```rust
#[tokio::test]
async fn test_risk_classifier_with_mock() {
    // Create a mock model that returns predetermined responses
    let mock_model = create_mock_model(vec![
        r#"{"level": 1, "reason": "safe command"}"#,
        r#"{"level": 4, "reason": "dangerous command"}"#,
    ]);
    
    let classifier = RiskClassifier::new(mock_model);
    
    let result = classifier.evaluate("ls -la", "/home/user", &[]).await.unwrap();
    assert_eq!(result.level, RiskLevel::Allowed);
    
    let result = classifier.evaluate("rm -rf /", "/home/user", &[]).await.unwrap();
    assert_eq!(result.level, RiskLevel::Catastrophic);
}
```

---

## Testing Config Loading

```rust
#[tokio::test]
async fn test_config_env_override() {
    // Use temp_env or similar crate for env var testing
    temp_env::with_vars([
        ("SHAI_PROVIDER", Some("anthropic")),
        ("SHAI_MODEL", Some("claude-3-5-sonnet")),
    ], || {
        let config = Config::load().unwrap();
        assert_eq!(config.provider.r#type, "anthropic");
        assert_eq!(config.provider.model, "claude-3-5-sonnet");
    });
}
```

---

## Testing History Serialization

```rust
#[test]
fn test_history_round_trip() {
    let dir = tempdir().unwrap();
    let path = dir.path().join("session.jsonl");
    
    let messages = vec![
        Message::user("hello"),
        Message::assistant("hi"),
        Message::tool_result("call-1", "tool output"),
    ];
    
    History::save(&path, &messages).unwrap();
    let loaded = History::load(&path, 50).unwrap();
    
    assert_eq!(loaded.len(), 3);
    assert_eq!(loaded[0].role, Role::User);
    assert_eq!(loaded[1].role, Role::Assistant);
    assert_eq!(loaded[2].role, Role::Tool);
}
```

---

## Testing with Debug Output

```rust
#[tokio::test]
async fn test_with_debug_output() {
    // Enable debug for this test
    std::env::set_var("SHAI_DEBUG", "1");
    
    let result = run_agent("echo test", &config).await;
    
    // Debug output will be printed during test execution
    assert!(result.is_ok());
}
```

---

## Common Test Utilities

```rust
// In a test_util module or at top of test file
fn create_test_file(content: &str) -> (tempfile::TempDir, PathBuf) {
    let cwd = env::current_dir().unwrap();
    let dir = Builder::new().prefix("shai_test_").tempdir_in(&cwd).unwrap();
    let file = dir.path().join("test.txt");
    fs::write(&file, content).unwrap();
    (dir, file)
}

fn assert_tool_success(result: Result<ToolResult, ToolError>) {
    assert!(result.is_ok());
    let tool_result = result.unwrap();
    assert!(tool_result.is_success());
}

fn assert_tool_error(result: Result<ToolResult, ToolError>, expected_msg: &str) {
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(err.to_string().contains(expected_msg));
}
```

---

## References

- [Testing Philosophy](./philosophy.md)
- [Test Organization](./organization.md)
- [Component Strategies](./components.md)
- [Testing Strategy](../strategy.md)