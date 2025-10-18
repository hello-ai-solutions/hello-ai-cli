use serde_json::{json, Value};
use std::fs;
use std::path::Path;

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct TodoList {
    pub action: String,
    pub task: Option<String>,
    pub list_id: Option<String>,
    pub completed: Option<bool>,
}

impl TodoList {
    pub async fn execute(&self) -> Result<String, Box<dyn std::error::Error>> {
        let todo_dir = ".amazonq/cli-todo-lists";
        fs::create_dir_all(todo_dir)?;

        match self.action.as_str() {
            "create" => {
                if let Some(task) = &self.task {
                    let list_id = format!("todo_{}", chrono::Utc::now().timestamp());
                    let todo_file = format!("{}/{}.json", todo_dir, list_id);

                    let todo_data = json!({
                        "id": list_id,
                        "tasks": [{"task": task, "completed": false}],
                        "created": chrono::Utc::now().to_rfc3339()
                    });

                    fs::write(&todo_file, serde_json::to_string_pretty(&todo_data)?)?;
                    Ok(format!("Created TODO list: {}\n[ ] {}", list_id, task))
                } else {
                    Err("Missing task for TODO creation".into())
                }
            }
            "add" => {
                if let (Some(list_id), Some(task)) = (&self.list_id, &self.task) {
                    let todo_file = format!("{}/{}.json", todo_dir, list_id);
                    if Path::new(&todo_file).exists() {
                        let data = fs::read_to_string(&todo_file)?;
                        let mut todo_data: Value = serde_json::from_str(&data)?;

                        if let Some(tasks) = todo_data["tasks"].as_array_mut() {
                            tasks.push(json!({"task": task, "completed": false}));
                        }

                        fs::write(&todo_file, serde_json::to_string_pretty(&todo_data)?)?;
                        Ok(format!("Added task to {}: {}", list_id, task))
                    } else {
                        Err(format!("TODO list {} not found", list_id).into())
                    }
                } else {
                    Err("Missing list_id or task for TODO addition".into())
                }
            }
            "complete" => {
                if let (Some(list_id), Some(task)) = (&self.list_id, &self.task) {
                    let todo_file = format!("{}/{}.json", todo_dir, list_id);
                    if Path::new(&todo_file).exists() {
                        let data = fs::read_to_string(&todo_file)?;
                        let mut todo_data: Value = serde_json::from_str(&data)?;

                        if let Some(tasks) = todo_data["tasks"].as_array_mut() {
                            for task_obj in tasks {
                                if task_obj["task"].as_str() == Some(task) {
                                    task_obj["completed"] = json!(true);
                                    break;
                                }
                            }
                        }

                        fs::write(&todo_file, serde_json::to_string_pretty(&todo_data)?)?;
                        Ok(format!("Completed task: {}", task))
                    } else {
                        Err(format!("TODO list {} not found", list_id).into())
                    }
                } else {
                    Err("Missing list_id or task for TODO completion".into())
                }
            }
            "list" => {
                let entries = fs::read_dir(todo_dir)?;
                let mut todos = Vec::new();

                for entry in entries {
                    let entry = entry?;
                    if entry.path().extension().and_then(|s| s.to_str()) == Some("json") {
                        let data = fs::read_to_string(entry.path())?;
                        let todo_data: Value = serde_json::from_str(&data)?;

                        if let Some(tasks) = todo_data["tasks"].as_array() {
                            let mut task_list = Vec::new();
                            for task in tasks {
                                let completed = task["completed"].as_bool().unwrap_or(false);
                                let task_text = task["task"].as_str().unwrap_or("Unknown");
                                let status = if completed { "[x]" } else { "[ ]" };
                                task_list.push(format!("{} {}", status, task_text));
                            }
                            todos.push(format!(
                                "TODO {}:\n{}",
                                todo_data["id"].as_str().unwrap_or("unknown"),
                                task_list.join("\n")
                            ));
                        }
                    }
                }

                if todos.is_empty() {
                    Ok("No TODO lists found".to_string())
                } else {
                    Ok(todos.join("\n\n"))
                }
            }
            _ => Err(format!("Unknown TODO action: {}", self.action).into()),
        }
    }
}
