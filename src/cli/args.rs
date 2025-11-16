use crate::core::task::Priority;
use clap::{Parser, Subcommand, ValueEnum};

#[derive(Parser)]
#[command(name = "todo")]
#[command(about="A modern todo list manager", long_about=None)]
#[command(version)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Add a new task
    Add {
        /// The task name
        name: String,

        /// Priority of the task
        #[arg(short, long, value_enum)]
        priority: Option<Priority>,

        /// Task category
        #[arg(short, long)]
        category: Option<String>,

        /// Due date (YYYY-MM-DD or +ndays)
        #[arg(short, long)]
        due: Option<String>,

        /// Tags (comma-separated)
        #[arg(short, long)]
        tags: Option<String>,
    },
    /// List all tasks
    List {
        /// Filter by status
        #[arg(short, long, value_enum)]
        status: Option<StatusFilter>,

        /// Filter by priority
        #[arg(short, long)]
        priority: Option<PriorityArg>,

        /// Filter by category
        #[arg(short, long)]
        category: Option<String>,

        /// Show only overdue tasks
        #[arg(short, long)]
        overdue: bool,

        /// Show only tasks due today
        #[arg(short, long)]
        today: bool,
    },

    /// Edit an existing task
    Edit {
        /// Task ID
        id: u64,
        /// New name
        #[arg(short, long)]
        name: Option<String>,

        /// New priority
        #[arg(short, long, value_enum)]
        priority: Option<PriorityArg>,

        /// New category
        #[arg(short, long)]
        category: Option<String>,

        /// New due date
        #[arg(short, long)]
        due: Option<String>,
    },

    /// Mark a task as completed
    Complete {
        /// The ID of the task to complete
        id: u64,
    },

    /// Remove a task
    Remove {
        /// The ID of the task to remove
        id: u64,
    },

    Category {
        #[command(subcommand)]
        command: CategoryCommands,
    },
}

#[derive(Subcommand)]
pub enum CategoryCommands {
    /// List all categories
    List,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum PriorityArg {
    Low,
    Medium,
    High,
    Critical,
}

impl From<PriorityArg> for Priority {
    fn from(value: PriorityArg) -> Self {
        match value {
            PriorityArg::Low => Priority::Low,
            PriorityArg::Medium => Priority::Medium,
            PriorityArg::High => Priority::High,
            PriorityArg::Critical => Priority::Critical,
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum StatusFilter {
    Pending,
    Completed,
    All,
}
