use crate::cli::args::{CategoryCommands, Cli, Commands, StatusFilter};
use crate::core::store::TodoManager;
use crate::core::task::Status;
use crate::ui::display::{
    display_info, display_success, display_task_table, display_welcome, PrettyTask,
};
use anyhow::Result;
use chrono::Utc;

pub mod args;

impl Cli {
    pub fn execute(self, manager: &mut TodoManager) -> Result<()> {
        match self.command {
            Commands::Add {
                name,
                priority,
                category,
                due,
                tags,
            } => {
                let id = manager.add_task(
                    &name,
                    priority,
                    category.as_deref(),
                    due.as_deref(),
                    tags.as_deref(),
                )?;
                display_welcome();
                display_success(&format!("Task added successfully! (ID: {})", id))
            }
            Commands::List {
                status,
                priority,
                category,
                overdue,
                today,
            } => {
                display_welcome();

                let mut tasks = manager.list_tasks();

                if let Some(status_filter) = status {
                    tasks.retain(|task| match status_filter {
                        StatusFilter::Pending => task.status == Status::Pending,
                        StatusFilter::Completed => task.status == Status::Completed,
                        StatusFilter::All => true,
                    })
                }

                if let Some(priority_filter) = priority {
                    let priority = priority_filter.into();
                    tasks.retain(|task| task.priority == priority);
                }

                if let Some(cat_filter) = category {
                    tasks.retain(|task| task.category == cat_filter);
                }

                if overdue {
                    tasks.retain(|task| task.is_overdue());
                }

                if today {
                    tasks.retain(|task| {
                        if let Some(due) = task.due_date {
                            let now = Utc::now();
                            due.date_naive() == now.date_naive()
                        } else {
                            false
                        }
                    })
                }

                if tasks.is_empty() {
                    display_info("No tasks found");
                } else {
                    display_task_table(&tasks);
                }
            }
            Commands::Edit {
                id,
                name,
                priority,
                category,
                due,
            } => {
                let priority = priority.map(|p| p.into());
                manager.edit_task(id, name, priority, category, due)?;

                display_welcome();
                display_success(&format!("Task {} updated successfully!", id));

                if let Some(task) = manager.get_task(id) {
                    println!("  {}", PrettyTask(task));
                }
            }
            Commands::Complete { id } => {
                manager.complete_task(id)?;
                display_welcome();
                display_success(&format!("Task {} marked as completed!", id));
            }
            Commands::Remove { id } => {
                manager.remove_task(id)?;
                display_welcome();
                display_success(&format!("Task {} removed successfully!", id));
            }
            Commands::Category { command } => {
                display_welcome();

                match command {
                    CategoryCommands::List => {
                        let categories = manager.get_categories();
                        if categories.is_empty() {
                            display_info("No categories found");
                        } else {
                            println!("Categories:");
                            for cat in categories {
                                let tasks = manager.get_tasks_by_category(&cat);
                                let completed = tasks
                                    .iter()
                                    .filter(|t| t.status == Status::Completed)
                                    .count();
                                println!(
                                    "  • {} ({} tasks, {} completed)",
                                    cat,
                                    tasks.len(),
                                    completed
                                );
                            }
                        }
                    }
                }
            }
        }
        Ok(())
    }
}
