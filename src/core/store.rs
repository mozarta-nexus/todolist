use crate::core::task::{Priority, Task};
use crate::error::{Result, TodoError};
use chrono::{DateTime, Duration, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

const DATA_FILE: &str = "tasks.json";

#[derive(Debug, Serialize, Deserialize)]
pub struct TodoManager {
    tasks: HashMap<u64, Task>,
    next_id: u64,
}

impl TodoManager {
    pub fn new() -> Result<Self> {
        let mut manager = Self {
            tasks: HashMap::new(),
            next_id: 1,
        };
        manager.load()?;
        Ok(manager)
    }

    fn load(&mut self) -> Result<()> {
        let path = self.data_path()?;
        if path.exists() {
            let data = fs::read_to_string(path)?;
            let tasks: HashMap<u64, Task> = serde_json::from_str(&data)?;
            self.tasks = tasks;
            self.next_id = self.tasks.keys().max().unwrap_or(&0) + 1;
        }
        Ok(())
    }

    fn data_path(&self) -> Result<PathBuf> {
        let mut path = dirs::data_dir().ok_or_else(|| {
            std::io::Error::new(std::io::ErrorKind::NotFound, "Data directory not found")
        })?;
        path.push("todolist");
        path.push(DATA_FILE);
        Ok(path)
    }

    pub fn add_task(
        &mut self,
        name: &str,
        priority: Option<Priority>,
        category: Option<&str>,
        due_date: Option<&str>,
        tags: Option<&str>,
    ) -> Result<u64> {
        let mut task = Task::new(self.next_id, name.to_string());
        if let Some(pri) = priority {
            task.set_priority(pri);
        }

        if let Some(cat) = category {
            task.set_category(cat.to_string())
        }

        if let Some(due) = due_date {
            if let Some(parsed_due) = Self::parse_due_date(due)? {
                task.set_due_date(parsed_due);
            }
        }

        if let Some(tags_str) = tags {
            for tag in tags_str.split(',') {
                task.add_tag(tag.trim().to_string())
            }
        }

        let id = self.next_id;
        self.tasks.insert(id, task);
        self.next_id += 1;
        self.save()?;
        Ok(id)
    }

    pub fn edit_task(
        &mut self,
        id: u64,
        name: Option<String>,
        priority: Option<Priority>,
        category: Option<String>,
        due_date: Option<String>,
    ) -> Result<()> {
        if let Some(task) = self.tasks.get_mut(&id) {
            if let Some(na) = name {
                task.name = na;
                task.update_at = Utc::now();
            }

            if let Some(pri) = priority {
                task.set_priority(pri);
            }

            if let Some(cat) = category {
                task.set_category(cat.to_string())
            }

            if let Some(due) = due_date {
                let parsed_due = if due.is_empty() {
                    None
                } else {
                    Self::parse_due_date(&due)?
                };
                task.due_date = parsed_due;
                task.update_at = Utc::now();
            }
            self.save()?;
            Ok(())
        } else {
            Err(TodoError::TaskNotFound(id))
        }
    }

    pub fn list_tasks(&self) -> Vec<&Task> {
        let mut tasks: Vec<&Task> = self.tasks.values().collect();
        tasks.sort_by_key(|task| task.id);
        tasks
    }

    pub fn get_task(&self, id: u64) -> Option<&Task> {
        self.tasks.get(&id)
    }

    pub fn complete_task(&mut self, id: u64) -> Result<()> {
        if let Some(task) = self.tasks.get_mut(&id) {
            task.complete();
            self.save()?;
            Ok(())
        } else {
            Err(TodoError::TaskNotFound(id))
        }
    }

    pub fn remove_task(&mut self, id: u64) -> Result<()> {
        if self.tasks.remove(&id).is_some() {
            self.save()?;
            Ok(())
        } else {
            Err(TodoError::TaskNotFound(id))
        }
    }

    pub fn get_tasks_by_category(&self, category: &str) -> Vec<&Task> {
        self.tasks
            .values()
            .filter(|task| task.category == category)
            .collect()
    }

    pub fn get_categories(&self) -> Vec<String> {
        let mut categories: Vec<String> = self
            .tasks
            .values()
            .map(|task| task.category.clone())
            .collect();
        categories.sort();
        categories.dedup();
        categories
    }

    fn save(&self) -> Result<()> {
        let path = self.data_path()?;
        let data = serde_json::to_string_pretty(&self.tasks)?;
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        fs::write(path, data)?;
        Ok(())
    }

    fn parse_due_date(due: &str) -> Result<Option<DateTime<Utc>>> {
        if due.is_empty() {
            return Ok(None);
        }

        if due.starts_with('+') {
            if let Ok(days) = due[1..].trim_end_matches("days").trim().parse::<i64>() {
                let due_date = Utc::now() + Duration::days(days);
                return Ok(Some(due_date));
            }
        }

        if let Ok(native_date) = NaiveDate::parse_from_str(due, "%Y-%m-%d") {
            let due_date = native_date
                .and_hms_opt(23, 59, 59)
                .unwrap()
                .and_local_timezone(Utc)
                .unwrap();
            return Ok(Some(due_date));
        }

        Err(TodoError::InvalidDate(due.to_string()))
    }
}
