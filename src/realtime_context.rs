#![allow(dead_code)]
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use std::time::{SystemTime, UNIX_EPOCH};
use notify::{Watcher, RecursiveMode, watcher, DebouncedEvent};
use std::sync::mpsc::channel;
use std::time::Duration;

pub struct RealtimeContext {
    file_states: Arc<Mutex<HashMap<PathBuf, FileState>>>,
    cursor_positions: Arc<Mutex<HashMap<PathBuf, CursorPosition>>>,
    watcher: Option<notify::RecommendedWatcher>,
}

#[derive(Clone, Debug)]
#[allow(dead_code)]
struct FileState {
    content: String,
    last_modified: u64,
    language: String,
}

#[derive(Clone, Debug)]
#[allow(dead_code)]
struct CursorPosition {
    line: usize,
    column: usize,
    timestamp: u64,
}

impl RealtimeContext {
    pub fn new() -> Self {
        Self {
            file_states: Arc::new(Mutex::new(HashMap::new())),
            cursor_positions: Arc::new(Mutex::new(HashMap::new())),
            watcher: None,
        }
    }

    pub fn start_watching(&mut self, workspace_path: &str) -> Result<(), Box<dyn std::error::Error>> {
        let (tx, rx) = channel();
        let mut watcher = watcher(tx, Duration::from_secs(1))?;
        watcher.watch(workspace_path, RecursiveMode::Recursive)?;

        let file_states = Arc::clone(&self.file_states);
        
        std::thread::spawn(move || {
            loop {
                match rx.recv() {
                    Ok(event) => {
                        if let DebouncedEvent::Write(path) = event {
                            if let Ok(content) = std::fs::read_to_string(&path) {
                                let language = detect_language_from_path(&path);
                                let timestamp = SystemTime::now()
                                    .duration_since(UNIX_EPOCH)
                                    .unwrap()
                                    .as_secs();

                                let mut states = file_states.lock().unwrap();
                                states.insert(path, FileState {
                                    content,
                                    last_modified: timestamp,
                                    language,
                                });
                            }
                        }
                    }
                    Err(e) => println!("Watch error: {:?}", e),
                }
            }
        });

        self.watcher = Some(watcher);
        Ok(())
    }

    pub fn update_cursor_position(&self, file_path: &str, line: usize, column: usize) {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        let mut positions = self.cursor_positions.lock().unwrap();
        positions.insert(PathBuf::from(file_path), CursorPosition {
            line,
            column,
            timestamp,
        });
    }

    pub fn get_live_context(&self, file_path: &str, context_lines: usize) -> Option<String> {
        let states = self.file_states.lock().unwrap();
        let positions = self.cursor_positions.lock().unwrap();

        let path = PathBuf::from(file_path);
        let file_state = states.get(&path)?;
        let cursor_pos = positions.get(&path)?;

        let lines: Vec<&str> = file_state.content.lines().collect();
        let start = cursor_pos.line.saturating_sub(context_lines);
        let end = std::cmp::min(lines.len(), cursor_pos.line + context_lines + 1);

        Some(lines[start..end].join("\n"))
    }

    pub fn get_recent_changes(&self, _file_path: &str, since_seconds: u64) -> Vec<String> {
        let states = self.file_states.lock().unwrap();
        let current_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        states
            .iter()
            .filter(|(_, state)| current_time - state.last_modified <= since_seconds)
            .map(|(path, _)| path.to_string_lossy().to_string())
            .collect()
    }

    pub fn get_workspace_context(&self) -> HashMap<String, String> {
        let states = self.file_states.lock().unwrap();
        states
            .iter()
            .map(|(path, state)| {
                (path.to_string_lossy().to_string(), state.language.clone())
            })
            .collect()
    }
}

fn detect_language_from_path(path: &PathBuf) -> String {
    match path.extension().and_then(|s| s.to_str()) {
        Some("rs") => "rust".to_string(),
        Some("py") => "python".to_string(),
        Some("js") => "javascript".to_string(),
        Some("ts") => "typescript".to_string(),
        Some("java") => "java".to_string(),
        Some("cs") => "csharp".to_string(),
        Some("php") => "php".to_string(),
        Some("rb") => "ruby".to_string(),
        Some("swift") => "swift".to_string(),
        Some("go") => "go".to_string(),
        _ => "unknown".to_string(),
    }
}
