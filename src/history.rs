use anyhow::Result;
use rig::completion::Message;
use std::fs;
use std::path::Path;

pub struct History;

impl History {
    pub fn load(path: &Path, max_turns: usize) -> Result<Vec<Message>> {
        if !path.exists() {
            return Ok(Vec::new());
        }
        let content = fs::read_to_string(path)?;
        let messages: Vec<Message> = content
            .lines()
            .filter_map(|line| serde_json::from_str(line).ok())
            .collect();

        // Keep last max_turns * 4 messages (roughly max_turns turns)
        let keep = max_turns * 4;
        if messages.len() > keep {
            Ok(messages[messages.len() - keep..].to_vec())
        } else {
            Ok(messages)
        }
    }

    pub fn save(path: &Path, messages: &[Message]) -> Result<()> {
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        let content = messages
            .iter()
            .map(|m| serde_json::to_string(m).unwrap())
            .collect::<Vec<_>>()
            .join("\n");
        fs::write(path, content)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rig::completion::Message;
    use tempfile::tempdir;

    #[test]
    fn test_history_load_save() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("session.jsonl");

        let msg1 = Message::user("hello");
        let msg2 = Message::assistant("hi");

        History::save(&path, &[msg1.clone(), msg2.clone()]).unwrap();
        let loaded = History::load(&path, 50).unwrap();

        assert_eq!(loaded.len(), 2);
    }

    #[test]
    fn test_history_load_nonexistent() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("nonexistent.jsonl");
        let loaded = History::load(&path, 50).unwrap();
        assert!(loaded.is_empty());
    }

    #[test]
    fn test_history_turn_limit() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("session.jsonl");

        let mut msgs = Vec::new();
        for i in 0..100 {
            msgs.push(Message::user(format!("msg {}", i)));
        }

        History::save(&path, &msgs).unwrap();
        let loaded = History::load(&path, 10).unwrap(); // max 10 turns = 40 messages

        assert!(loaded.len() <= 40);
    }
}