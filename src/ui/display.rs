use crate::core::task::{Priority, Status, Task};
use chrono::Utc;
use colored::Colorize;
use prettytable::{Cell, Row, Table};
use std::fmt::Display;

pub struct PrettyTask<'a>(pub &'a Task);

impl<'a> Display for PrettyTask<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        let task = self.0;

        let status_icon = match task.status {
            Status::Pending => "◯".yellow().bold(),
            Status::Completed => "✔".green().bold(),
        };

        let priority_icon = match task.priority {
            Priority::Low => "⬇".blue(),
            Priority::Medium => "➡".cyan(),
            Priority::High => "⬆".yellow(),
            Priority::Critical => "⚠".red().bold(),
        };
        let task_name = if task.status == Status::Completed {
            task.name.strikethrough().truecolor(128, 128, 128)
        } else if task.is_overdue() {
            task.name.red().bold()
        } else {
            task.name.white()
        };
        let category = format!("[{}]", task.category).truecolor(180, 180, 180);

        let due_info = if let Some(due) = task.due_date {
            let now = Utc::now();
            if task.is_overdue() {
                "OVERDUE".red().bold()
            } else {
                let days = (due - now).num_days();
                if days == 0 {
                    "Today".yellow().bold()
                } else if days == 1 {
                    "Tomorrow".yellow()
                } else if days < 0 {
                    format!("{} days ago", days.abs()).red()
                } else {
                    format!("in {} days", days).green()
                }
            }
        } else {
            "No due date".truecolor(128, 128, 128)
        };

        let tags = if task.tags.is_empty() {
            "".normal()
        } else {
            format!(" 🏷 {}", task.tags.join(", ")).truecolor(200, 200, 100)
        };
        write!(
            fmt,
            "{} {} {}{} {} {}",
            status_icon, priority_icon, task_name, category, due_info, tags
        )
    }
}

pub fn display_task_table(tasks: &[&Task]) {
    let mut table = Table::new();

    // table head
    table.set_titles(Row::new(vec![
        Cell::new("ID").style_spec("bFg"),
        Cell::new("Status").style_spec("bFg"),
        Cell::new("Priority").style_spec("bFg"),
        Cell::new("Task").style_spec("bFg"),
        Cell::new("Category").style_spec("bFg"),
        Cell::new("Due Date").style_spec("bFg"),
        Cell::new("Tags").style_spec("bFg"),
    ]));

    for task in tasks {
        let status_cell = match task.status {
            Status::Pending => Cell::new("PENDING").style_spec("Fy"),
            Status::Completed => Cell::new("COMPLETED").style_spec("Fg"),
        };

        let priority_cell = match task.priority {
            Priority::Low => Cell::new("LOW").style_spec("Fb"),
            Priority::Medium => Cell::new("MEDIUM").style_spec("Fc"),
            Priority::High => Cell::new("HIGH").style_spec("Fy"),
            Priority::Critical => Cell::new("CRITICAL").style_spec("Fr"),
        };

        let due_info = if let Some(due) = task.due_date {
            let now = Utc::now();
            if task.is_overdue() {
                Cell::new("OVERDUE").style_spec("Fr")
            } else {
                let days = (due - now).num_days();
                if days == 0 {
                    Cell::new("TODAY").style_spec("Fy")
                } else if days > 0 {
                    Cell::new(&format!("in {} days", days)).style_spec("Fg")
                } else {
                    Cell::new(&format!("{} days ago", days.abs())).style_spec("Fr")
                }
            }
        } else {
            Cell::new("-")
        };

        let tags = if task.tags.is_empty() {
            "-".to_string()
        } else {
            task.tags.join(", ")
        };

        table.add_row(Row::new(vec![
            Cell::new(&task.id.to_string()),
            status_cell,
            priority_cell,
            Cell::new(&task.name),
            Cell::new(&task.category),
            due_info,
            Cell::new(&tags),
        ]));
    }
    table.printstd();
}

pub fn display_welcome() {
    println!("{}", "✨ Todo List Manager".bright_cyan().bold());
    println!("{}", "=".repeat(40).bright_black());
}

pub fn display_success(message: &str) {
    println!("{} {}", "✅".green(), message.green());
}

pub fn display_error(message: &str) {
    println!("{} {}", "❌".red(), message.red());
}

pub fn display_info(message: &str) {
    println!("{} {}", "ℹ️".blue(), message.blue());
}
