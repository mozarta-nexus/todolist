use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::fmt;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    pub id: u64,
    pub name: String,
    pub status: Status,
    pub priority: Priority,
    pub category: String,
    pub create_at: DateTime<Utc>,
    pub update_at: DateTime<Utc>,
    pub completed_at: Option<DateTime<Utc>>,
    pub due_date: Option<DateTime<Utc>>,
    pub tags: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Status {
    Pending,
    Completed,
}

impl fmt::Display for Status {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Status::Pending => write!(f, "Pending"),
            Status::Completed => write!(f, "Completed"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Priority {
    Low,
    Medium,
    High,
    Critical,
}

impl fmt::Display for Priority {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Priority::Low => write!(f, "Low"),
            Priority::Medium => write!(f, "Medium"),
            Priority::High => write!(f, "High"),
            Priority::Critical => write!(f, "Critical"),
        }
    }
}

impl std::str::FromStr for Priority {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "low" | "l" => Ok(Priority::Low),
            "medium" | "med" => Ok(Priority::Medium),
            "high" | "h" => Ok(Priority::High),
            _ => Err("Invalid priority".to_string()),
        }
    }
}

impl Task {
    pub fn new(id: u64, name: String) -> Self {
        let now = Utc::now();
        Self {
            id,
            name,
            status: Status::Pending,
            priority: Priority::Medium,
            category: "General".to_string(),
            create_at: now,
            update_at: now,
            completed_at: None,
            due_date: None,
            tags: vec![],
        }
    }

    pub fn complete(&mut self) {
        self.status = Status::Completed;
        self.completed_at = Some(Utc::now());
        self.update_at = Utc::now();
    }

    pub fn set_priority(&mut self, priority: Priority) {
        self.priority = priority;
        self.update_at = Utc::now();
    }

    pub fn set_category(&mut self, category: String) {
        self.category = category;
        self.update_at = Utc::now();
    }

    pub fn set_due_date(&mut self, due_date: DateTime<Utc>) {
        self.due_date = Some(due_date);
        self.update_at = Utc::now();
    }

    pub fn add_tag(&mut self, tag: String) {
        if !self.tags.contains(&tag) {
            self.tags.push(tag);
            self.update_at = Utc::now();
        }
    }

    pub fn is_overdue(&self) -> bool {
        if let Some(due) = self.due_date {
            self.status == Status::Pending && due < Utc::now()
        } else {
            false
        }
    }

    pub fn days_until_due(&self) -> Option<i64> {
        self.due_date.map(|due| {
            let now = Utc::now();
            let duration = due - now;
            duration.num_days()
        })
    }
}

impl Display for Task {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let status_icon = match self.status {
            Status::Pending => "  ",
            Status::Completed => "✅️",
        };
        let priority_icon = match self.priority {
            Priority::Low => "⬇",
            Priority::Medium => "➡",
            Priority::High => "⬆",
            Priority::Critical => "⚠",
        };

        write!(
            f,
            "#{} [{}{}] {}",
            self.id, status_icon, priority_icon, self.name
        )
    }
}
